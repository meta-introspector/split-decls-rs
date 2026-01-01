/* FP:stable_hasher.rs-0001 */ use std::hash::{BuildHasher, Hash, Hasher};
/* FP:stable_hasher.rs-0002 */ use std::marker::PhantomData;
/* FP:stable_hasher.rs-0003 */ use std::mem;
/* FP:stable_hasher.rs-0004 */ use std::num::NonZero;
/* FP:stable_hasher.rs-0005 */ 
/* FP:stable_hasher.rs-0006 */ use crate::rustc_index::bit_set::{self, DenseBitSet};
/* FP:stable_hasher.rs-0007 */ use crate::rustc_index::{Idx, IndexSlice, IndexVec};
/* FP:stable_hasher.rs-0008 */ use smallvec::SmallVec;
/* FP:stable_hasher.rs-0009 */ 
/* FP:stable_hasher.rs-0010 */ #[cfg(test)]
/* FP:stable_hasher.rs-0012 */ 
/* FP:stable_hasher.rs-0013 */ use rustc_hashes::{Hash64, Hash128};
/* FP:stable_hasher.rs-0014 */ pub use rustc_stable_hash::{
/* FP:stable_hasher.rs-0015 */     FromStableHash, SipHasher128Hash as StableHasherHash, StableSipHasher128 as StableHasher,
/* FP:stable_hasher.rs-0016 */ };
/* FP:stable_hasher.rs-0017 */ 
/* FP:stable_hasher.rs-0018 */ /// Something that implements `HashStable<CTX>` can be hashed in a way that is
/* FP:stable_hasher.rs-0019 */ /// stable across multiple compilation sessions.
/* FP:stable_hasher.rs-0020 */ ///
/* FP:stable_hasher.rs-0021 */ /// Note that `HashStable` imposes rather more strict requirements than usual
/* FP:stable_hasher.rs-0022 */ /// hash functions:
/* FP:stable_hasher.rs-0023 */ ///
/* FP:stable_hasher.rs-0024 */ /// - Stable hashes are sometimes used as identifiers. Therefore they must
/* FP:stable_hasher.rs-0025 */ ///   conform to the corresponding `PartialEq` implementations:
/* FP:stable_hasher.rs-0026 */ ///
/* FP:stable_hasher.rs-0027 */ ///     - `x == y` implies `hash_stable(x) == hash_stable(y)`, and
/* FP:stable_hasher.rs-0028 */ ///     - `x != y` implies `hash_stable(x) != hash_stable(y)`.
/* FP:stable_hasher.rs-0029 */ ///
/* FP:stable_hasher.rs-0030 */ ///   That second condition is usually not required for hash functions
/* FP:stable_hasher.rs-0031 */ ///   (e.g. `Hash`). In practice this means that `hash_stable` must feed any
/* FP:stable_hasher.rs-0032 */ ///   information into the hasher that a `PartialEq` comparison takes into
/* FP:stable_hasher.rs-0033 */ ///   account. See [#49300](https://github.com/rust-lang/rust/issues/49300)
/* FP:stable_hasher.rs-0034 */ ///   for an example where violating this invariant has caused trouble in the
/* FP:stable_hasher.rs-0035 */ ///   past.
/* FP:stable_hasher.rs-0036 */ ///
/* FP:stable_hasher.rs-0037 */ /// - `hash_stable()` must be independent of the current
/* FP:stable_hasher.rs-0038 */ ///    compilation session. E.g. they must not hash memory addresses or other
/* FP:stable_hasher.rs-0039 */ ///    things that are "randomly" assigned per compilation session.
/* FP:stable_hasher.rs-0040 */ ///
/* FP:stable_hasher.rs-0041 */ /// - `hash_stable()` must be independent of the host architecture. The
/* FP:stable_hasher.rs-0042 */ ///   `StableHasher` takes care of endianness and `isize`/`usize` platform
/* FP:stable_hasher.rs-0043 */ ///   differences.
/* FP:stable_hasher.rs-0044 */ pub trait HashStable<CTX> {
/* FP:stable_hasher.rs-0045 */     fn hash_stable(&self, hcx: &mut CTX, hasher: &mut StableHasher);
/* FP:stable_hasher.rs-0046 */ }
/* FP:stable_hasher.rs-0047 */ 
/* FP:stable_hasher.rs-0048 */ /// Implement this for types that can be turned into stable keys like, for
/* FP:stable_hasher.rs-0049 */ /// example, for DefId that can be converted to a DefPathHash. This is used for
/* FP:stable_hasher.rs-0050 */ /// bringing maps into a predictable order before hashing them.
/* FP:stable_hasher.rs-0051 */ pub trait ToStableHashKey<HCX> {
/* FP:stable_hasher.rs-0052 */     type KeyType: Ord + Sized + HashStable<HCX>;
/* FP:stable_hasher.rs-0053 */     fn to_stable_hash_key(&self, hcx: &HCX) -> Self::KeyType;
/* FP:stable_hasher.rs-0054 */ }
/* FP:stable_hasher.rs-0055 */ 
/* FP:stable_hasher.rs-0056 */ /// Trait for marking a type as having a sort order that is
/* FP:stable_hasher.rs-0057 */ /// stable across compilation session boundaries. More formally:
/* FP:stable_hasher.rs-0058 */ ///
/* FP:stable_hasher.rs-0059 */ /// ```txt
/* FP:stable_hasher.rs-0060 */ /// Ord::cmp(a1, b1) == Ord::cmp(a2, b2)
/* FP:stable_hasher.rs-0061 */ ///    where a2 = decode(encode(a1, context1), context2)
/* FP:stable_hasher.rs-0062 */ ///          b2 = decode(encode(b1, context1), context2)
/* FP:stable_hasher.rs-0063 */ /// ```
/* FP:stable_hasher.rs-0064 */ ///
/* FP:stable_hasher.rs-0065 */ /// i.e. the result of `Ord::cmp` is not influenced by encoding
/* FP:stable_hasher.rs-0066 */ /// the values in one session and then decoding them in another
/* FP:stable_hasher.rs-0067 */ /// session.
/* FP:stable_hasher.rs-0068 */ ///
/* FP:stable_hasher.rs-0069 */ /// This is trivially true for types where encoding and decoding
/* FP:stable_hasher.rs-0070 */ /// don't change the bytes of the values that are used during
/* FP:stable_hasher.rs-0071 */ /// comparison and comparison only depends on these bytes (as
/* FP:stable_hasher.rs-0072 */ /// opposed to some non-local state). Examples are u32, String,
/* FP:stable_hasher.rs-0073 */ /// Path, etc.
/* FP:stable_hasher.rs-0074 */ ///
/* FP:stable_hasher.rs-0075 */ /// But it is not true for:
/* FP:stable_hasher.rs-0076 */ ///  - `*const T` and `*mut T` because the values of these pointers
/* FP:stable_hasher.rs-0077 */ ///    will change between sessions.
/* FP:stable_hasher.rs-0078 */ ///  - `DefIndex`, `CrateNum`, `LocalDefId`, because their concrete
/* FP:stable_hasher.rs-0079 */ ///    values depend on state that might be different between
/* FP:stable_hasher.rs-0080 */ ///    compilation sessions.
/* FP:stable_hasher.rs-0081 */ ///
/* FP:stable_hasher.rs-0082 */ /// The associated constant `CAN_USE_UNSTABLE_SORT` denotes whether
/* FP:stable_hasher.rs-0083 */ /// unstable sorting can be used for this type. Set to true if and
/* FP:stable_hasher.rs-0084 */ /// only if `a == b` implies `a` and `b` are fully indistinguishable.
/* FP:stable_hasher.rs-0085 */ pub trait StableOrd: Ord {
/* FP:stable_hasher.rs-0086 */     const CAN_USE_UNSTABLE_SORT: bool;
/* FP:stable_hasher.rs-0087 */ 
/* FP:stable_hasher.rs-0088 */     /// Marker to ensure that implementors have carefully considered
/* FP:stable_hasher.rs-0089 */     /// whether their `Ord` implementation obeys this trait's contract.
/* FP:stable_hasher.rs-0090 */     const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: ();
/* FP:stable_hasher.rs-0091 */ }
/* FP:stable_hasher.rs-0092 */ 
/* FP:stable_hasher.rs-0093 */ impl<T: StableOrd> StableOrd for &T {
/* FP:stable_hasher.rs-0094 */     const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT;
/* FP:stable_hasher.rs-0095 */ 
/* FP:stable_hasher.rs-0096 */     // Ordering of a reference is exactly that of the referent, and since
/* FP:stable_hasher.rs-0097 */     // the ordering of the referet is stable so must be the ordering of the
/* FP:stable_hasher.rs-0098 */     // reference.
/* FP:stable_hasher.rs-0099 */     const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: () = ();
/* FP:stable_hasher.rs-0100 */ }
/* FP:stable_hasher.rs-0101 */ 
/* FP:stable_hasher.rs-0102 */ /// This is a companion trait to `StableOrd`. Some types like `Symbol` can be
/* FP:stable_hasher.rs-0103 */ /// compared in a cross-session stable way, but their `Ord` implementation is
/* FP:stable_hasher.rs-0104 */ /// not stable. In such cases, a `StableOrd` implementation can be provided
/* FP:stable_hasher.rs-0105 */ /// to offer a lightweight way for stable sorting. (The more heavyweight option
/* FP:stable_hasher.rs-0106 */ /// is to sort via `ToStableHashKey`, but then sorting needs to have access to
/* FP:stable_hasher.rs-0107 */ /// a stable hashing context and `ToStableHashKey` can also be expensive as in
/* FP:stable_hasher.rs-0108 */ /// the case of `Symbol` where it has to allocate a `String`.)
/* FP:stable_hasher.rs-0109 */ ///
/* FP:stable_hasher.rs-0110 */ /// See the documentation of [StableOrd] for how stable sort order is defined.
/* FP:stable_hasher.rs-0111 */ /// The same definition applies here. Be careful when implementing this trait.
/* FP:stable_hasher.rs-0112 */ pub trait StableCompare {
/* FP:stable_hasher.rs-0113 */     const CAN_USE_UNSTABLE_SORT: bool;
/* FP:stable_hasher.rs-0114 */ 
/* FP:stable_hasher.rs-0115 */     fn stable_cmp(&self, other: &Self) -> std::cmp::Ordering;
/* FP:stable_hasher.rs-0116 */ }
/* FP:stable_hasher.rs-0117 */ 
/* FP:stable_hasher.rs-0118 */ /// `StableOrd` denotes that the type's `Ord` implementation is stable, so
/* FP:stable_hasher.rs-0119 */ /// we can implement `StableCompare` by just delegating to `Ord`.
/* FP:stable_hasher.rs-0120 */ impl<T: StableOrd> StableCompare for T {
/* FP:stable_hasher.rs-0121 */     const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT;
/* FP:stable_hasher.rs-0122 */ 
/* FP:stable_hasher.rs-0123 */     fn stable_cmp(&self, other: &Self) -> std::cmp::Ordering {
/* FP:stable_hasher.rs-0124 */         self.cmp(other)
/* FP:stable_hasher.rs-0125 */     }
/* FP:stable_hasher.rs-0126 */ }
/* FP:stable_hasher.rs-0127 */ 
/* FP:stable_hasher.rs-0128 */ /// Implement HashStable by just calling `Hash::hash()`. Also implement `StableOrd` for the type since
/* FP:stable_hasher.rs-0129 */ /// that has the same requirements.
/* FP:stable_hasher.rs-0130 */ ///
/* FP:stable_hasher.rs-0131 */ /// **WARNING** This is only valid for types that *really* don't need any context for fingerprinting.
/* FP:stable_hasher.rs-0132 */ /// But it is easy to misuse this macro (see [#96013](https://github.com/rust-lang/rust/issues/96013)
/* FP:stable_hasher.rs-0133 */ /// for examples). Therefore this macro is not exported and should only be used in the limited cases
/* FP:stable_hasher.rs-0134 */ /// here in this module.
/* FP:stable_hasher.rs-0135 */ ///
/* FP:stable_hasher.rs-0136 */ /// Use `#[derive(HashStable_Generic)]` instead.
/* FP:stable_hasher.rs-0137 */ macro_rules! impl_stable_traits_for_trivial_type {
/* FP:stable_hasher.rs-0138 */     ($t:ty) => {
/* FP:stable_hasher.rs-0139 */         impl<CTX> $crate::stable_hasher::HashStable<CTX> for $t {
/* FP:stable_hasher.rs-0140 */             #[inline]
/* FP:stable_hasher.rs-0141 */             fn hash_stable(&self, _: &mut CTX, hasher: &mut $crate::stable_hasher::StableHasher) {
/* FP:stable_hasher.rs-0142 */                 ::std::hash::Hash::hash(self, hasher);
/* FP:stable_hasher.rs-0143 */             }
/* FP:stable_hasher.rs-0144 */         }
/* FP:stable_hasher.rs-0145 */ 
/* FP:stable_hasher.rs-0146 */         impl $crate::stable_hasher::StableOrd for $t {
/* FP:stable_hasher.rs-0147 */             const CAN_USE_UNSTABLE_SORT: bool = true;
/* FP:stable_hasher.rs-0148 */ 
/* FP:stable_hasher.rs-0149 */             // Encoding and decoding doesn't change the bytes of trivial types
/* FP:stable_hasher.rs-0150 */             // and `Ord::cmp` depends only on those bytes.
/* FP:stable_hasher.rs-0151 */             const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: () = ();
/* FP:stable_hasher.rs-0152 */         }
/* FP:stable_hasher.rs-0153 */     };
/* FP:stable_hasher.rs-0154 */ }
/* FP:stable_hasher.rs-0155 */ 
/* FP:stable_hasher.rs-0156 */ pub(crate) use impl_stable_traits_for_trivial_type;
/* FP:stable_hasher.rs-0157 */ 
/* FP:stable_hasher.rs-0158 */ impl_stable_traits_for_trivial_type!(i8);
/* FP:stable_hasher.rs-0159 */ impl_stable_traits_for_trivial_type!(i16);
/* FP:stable_hasher.rs-0160 */ impl_stable_traits_for_trivial_type!(i32);
/* FP:stable_hasher.rs-0161 */ impl_stable_traits_for_trivial_type!(i64);
/* FP:stable_hasher.rs-0162 */ impl_stable_traits_for_trivial_type!(isize);
/* FP:stable_hasher.rs-0163 */ 
/* FP:stable_hasher.rs-0164 */ impl_stable_traits_for_trivial_type!(u8);
/* FP:stable_hasher.rs-0165 */ impl_stable_traits_for_trivial_type!(u16);
/* FP:stable_hasher.rs-0166 */ impl_stable_traits_for_trivial_type!(u32);
/* FP:stable_hasher.rs-0167 */ impl_stable_traits_for_trivial_type!(u64);
/* FP:stable_hasher.rs-0168 */ impl_stable_traits_for_trivial_type!(usize);
/* FP:stable_hasher.rs-0169 */ 
/* FP:stable_hasher.rs-0170 */ impl_stable_traits_for_trivial_type!(u128);
/* FP:stable_hasher.rs-0171 */ impl_stable_traits_for_trivial_type!(i128);
/* FP:stable_hasher.rs-0172 */ 
/* FP:stable_hasher.rs-0173 */ impl_stable_traits_for_trivial_type!(char);
/* FP:stable_hasher.rs-0174 */ impl_stable_traits_for_trivial_type!(());
/* FP:stable_hasher.rs-0175 */ 
/* FP:stable_hasher.rs-0176 */ impl_stable_traits_for_trivial_type!(Hash64);
/* FP:stable_hasher.rs-0177 */ 
/* FP:stable_hasher.rs-0178 */ // We need a custom impl as the default hash function will only hash half the bits. For stable
/* FP:stable_hasher.rs-0179 */ // hashing we want to hash the full 128-bit hash.
/* FP:stable_hasher.rs-0180 */ impl<CTX> HashStable<CTX> for Hash128 {
/* FP:stable_hasher.rs-0181 */     #[inline]
/* FP:stable_hasher.rs-0182 */     fn hash_stable(&self, _: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0183 */         self.as_u128().hash(hasher);
/* FP:stable_hasher.rs-0184 */     }
/* FP:stable_hasher.rs-0185 */ }
/* FP:stable_hasher.rs-0186 */ 
/* FP:stable_hasher.rs-0187 */ impl StableOrd for Hash128 {
/* FP:stable_hasher.rs-0188 */     const CAN_USE_UNSTABLE_SORT: bool = true;
/* FP:stable_hasher.rs-0189 */ 
/* FP:stable_hasher.rs-0190 */     // Encoding and decoding doesn't change the bytes of `Hash128`
/* FP:stable_hasher.rs-0191 */     // and `Ord::cmp` depends only on those bytes.
/* FP:stable_hasher.rs-0192 */     const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: () = ();
/* FP:stable_hasher.rs-0193 */ }
/* FP:stable_hasher.rs-0194 */ 
/* FP:stable_hasher.rs-0195 */ impl<CTX> HashStable<CTX> for ! {
/* FP:stable_hasher.rs-0196 */     fn hash_stable(&self, _ctx: &mut CTX, _hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0197 */         unreachable!()
/* FP:stable_hasher.rs-0198 */     }
/* FP:stable_hasher.rs-0199 */ }
/* FP:stable_hasher.rs-0200 */ 
/* FP:stable_hasher.rs-0201 */ impl<CTX, T> HashStable<CTX> for PhantomData<T> {
/* FP:stable_hasher.rs-0202 */     fn hash_stable(&self, _ctx: &mut CTX, _hasher: &mut StableHasher) {}
/* FP:stable_hasher.rs-0203 */ }
/* FP:stable_hasher.rs-0204 */ 
/* FP:stable_hasher.rs-0205 */ impl<CTX> HashStable<CTX> for NonZero<u32> {
/* FP:stable_hasher.rs-0206 */     #[inline]
/* FP:stable_hasher.rs-0207 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0208 */         self.get().hash_stable(ctx, hasher)
/* FP:stable_hasher.rs-0209 */     }
/* FP:stable_hasher.rs-0210 */ }
/* FP:stable_hasher.rs-0211 */ 
/* FP:stable_hasher.rs-0212 */ impl<CTX> HashStable<CTX> for NonZero<usize> {
/* FP:stable_hasher.rs-0213 */     #[inline]
/* FP:stable_hasher.rs-0214 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0215 */         self.get().hash_stable(ctx, hasher)
/* FP:stable_hasher.rs-0216 */     }
/* FP:stable_hasher.rs-0217 */ }
/* FP:stable_hasher.rs-0218 */ 
/* FP:stable_hasher.rs-0219 */ impl<CTX> HashStable<CTX> for f32 {
/* FP:stable_hasher.rs-0220 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0221 */         let val: u32 = self.to_bits();
/* FP:stable_hasher.rs-0222 */         val.hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0223 */     }
/* FP:stable_hasher.rs-0224 */ }
/* FP:stable_hasher.rs-0225 */ 
/* FP:stable_hasher.rs-0226 */ impl<CTX> HashStable<CTX> for f64 {
/* FP:stable_hasher.rs-0227 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0228 */         let val: u64 = self.to_bits();
/* FP:stable_hasher.rs-0229 */         val.hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0230 */     }
/* FP:stable_hasher.rs-0231 */ }
/* FP:stable_hasher.rs-0232 */ 
/* FP:stable_hasher.rs-0233 */ impl<CTX> HashStable<CTX> for ::std::cmp::Ordering {
/* FP:stable_hasher.rs-0234 */     #[inline]
/* FP:stable_hasher.rs-0235 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0236 */         (*self as i8).hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0237 */     }
/* FP:stable_hasher.rs-0238 */ }
/* FP:stable_hasher.rs-0239 */ 
/* FP:stable_hasher.rs-0240 */ impl<T1: HashStable<CTX>, CTX> HashStable<CTX> for (T1,) {
/* FP:stable_hasher.rs-0241 */     #[inline]
/* FP:stable_hasher.rs-0242 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0243 */         let (ref _0,) = *self;
/* FP:stable_hasher.rs-0244 */         _0.hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0245 */     }
/* FP:stable_hasher.rs-0246 */ }
/* FP:stable_hasher.rs-0247 */ 
/* FP:stable_hasher.rs-0248 */ impl<T1: HashStable<CTX>, T2: HashStable<CTX>, CTX> HashStable<CTX> for (T1, T2) {
/* FP:stable_hasher.rs-0249 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0250 */         let (ref _0, ref _1) = *self;
/* FP:stable_hasher.rs-0251 */         _0.hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0252 */         _1.hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0253 */     }
/* FP:stable_hasher.rs-0254 */ }
/* FP:stable_hasher.rs-0255 */ 
/* FP:stable_hasher.rs-0256 */ impl<T1: StableOrd, T2: StableOrd> StableOrd for (T1, T2) {
/* FP:stable_hasher.rs-0257 */     const CAN_USE_UNSTABLE_SORT: bool = T1::CAN_USE_UNSTABLE_SORT && T2::CAN_USE_UNSTABLE_SORT;
/* FP:stable_hasher.rs-0258 */ 
/* FP:stable_hasher.rs-0259 */     // Ordering of tuples is a pure function of their elements' ordering, and since
/* FP:stable_hasher.rs-0260 */     // the ordering of each element is stable so must be the ordering of the tuple.
/* FP:stable_hasher.rs-0261 */     const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: () = ();
/* FP:stable_hasher.rs-0262 */ }
/* FP:stable_hasher.rs-0263 */ 
/* FP:stable_hasher.rs-0264 */ impl<T1, T2, T3, CTX> HashStable<CTX> for (T1, T2, T3)
/* FP:stable_hasher.rs-0265 */ where
/* FP:stable_hasher.rs-0266 */     T1: HashStable<CTX>,
/* FP:stable_hasher.rs-0267 */     T2: HashStable<CTX>,
/* FP:stable_hasher.rs-0268 */     T3: HashStable<CTX>,
/* FP:stable_hasher.rs-0269 */ {
/* FP:stable_hasher.rs-0270 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0271 */         let (ref _0, ref _1, ref _2) = *self;
/* FP:stable_hasher.rs-0272 */         _0.hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0273 */         _1.hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0274 */         _2.hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0275 */     }
/* FP:stable_hasher.rs-0276 */ }
/* FP:stable_hasher.rs-0277 */ 
/* FP:stable_hasher.rs-0278 */ impl<T1: StableOrd, T2: StableOrd, T3: StableOrd> StableOrd for (T1, T2, T3) {
/* FP:stable_hasher.rs-0279 */     const CAN_USE_UNSTABLE_SORT: bool =
/* FP:stable_hasher.rs-0280 */         T1::CAN_USE_UNSTABLE_SORT && T2::CAN_USE_UNSTABLE_SORT && T3::CAN_USE_UNSTABLE_SORT;
/* FP:stable_hasher.rs-0281 */ 
/* FP:stable_hasher.rs-0282 */     // Ordering of tuples is a pure function of their elements' ordering, and since
/* FP:stable_hasher.rs-0283 */     // the ordering of each element is stable so must be the ordering of the tuple.
/* FP:stable_hasher.rs-0284 */     const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: () = ();
/* FP:stable_hasher.rs-0285 */ }
/* FP:stable_hasher.rs-0286 */ 
/* FP:stable_hasher.rs-0287 */ impl<T1, T2, T3, T4, CTX> HashStable<CTX> for (T1, T2, T3, T4)
/* FP:stable_hasher.rs-0288 */ where
/* FP:stable_hasher.rs-0289 */     T1: HashStable<CTX>,
/* FP:stable_hasher.rs-0290 */     T2: HashStable<CTX>,
/* FP:stable_hasher.rs-0291 */     T3: HashStable<CTX>,
/* FP:stable_hasher.rs-0292 */     T4: HashStable<CTX>,
/* FP:stable_hasher.rs-0293 */ {
/* FP:stable_hasher.rs-0294 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0295 */         let (ref _0, ref _1, ref _2, ref _3) = *self;
/* FP:stable_hasher.rs-0296 */         _0.hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0297 */         _1.hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0298 */         _2.hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0299 */         _3.hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0300 */     }
/* FP:stable_hasher.rs-0301 */ }
/* FP:stable_hasher.rs-0302 */ 
/* FP:stable_hasher.rs-0303 */ impl<T1: StableOrd, T2: StableOrd, T3: StableOrd, T4: StableOrd> StableOrd for (T1, T2, T3, T4) {
/* FP:stable_hasher.rs-0304 */     const CAN_USE_UNSTABLE_SORT: bool = T1::CAN_USE_UNSTABLE_SORT
/* FP:stable_hasher.rs-0305 */         && T2::CAN_USE_UNSTABLE_SORT
/* FP:stable_hasher.rs-0306 */         && T3::CAN_USE_UNSTABLE_SORT
/* FP:stable_hasher.rs-0307 */         && T4::CAN_USE_UNSTABLE_SORT;
/* FP:stable_hasher.rs-0308 */ 
/* FP:stable_hasher.rs-0309 */     // Ordering of tuples is a pure function of their elements' ordering, and since
/* FP:stable_hasher.rs-0310 */     // the ordering of each element is stable so must be the ordering of the tuple.
/* FP:stable_hasher.rs-0311 */     const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: () = ();
/* FP:stable_hasher.rs-0312 */ }
/* FP:stable_hasher.rs-0313 */ 
/* FP:stable_hasher.rs-0314 */ impl<T: HashStable<CTX>, CTX> HashStable<CTX> for [T] {
/* FP:stable_hasher.rs-0315 */     default fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0316 */         self.len().hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0317 */         for item in self {
/* FP:stable_hasher.rs-0318 */             item.hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0319 */         }
/* FP:stable_hasher.rs-0320 */     }
/* FP:stable_hasher.rs-0321 */ }
/* FP:stable_hasher.rs-0322 */ 
/* FP:stable_hasher.rs-0323 */ impl<CTX> HashStable<CTX> for [u8] {
/* FP:stable_hasher.rs-0324 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0325 */         self.len().hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0326 */         hasher.write(self);
/* FP:stable_hasher.rs-0327 */     }
/* FP:stable_hasher.rs-0328 */ }
/* FP:stable_hasher.rs-0329 */ 
/* FP:stable_hasher.rs-0330 */ impl<T: HashStable<CTX>, CTX> HashStable<CTX> for Vec<T> {
/* FP:stable_hasher.rs-0331 */     #[inline]
/* FP:stable_hasher.rs-0332 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0333 */         self[..].hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0334 */     }
/* FP:stable_hasher.rs-0335 */ }
/* FP:stable_hasher.rs-0336 */ 
/* FP:stable_hasher.rs-0337 */ impl<K, V, R, CTX> HashStable<CTX> for indexmap::IndexMap<K, V, R>
/* FP:stable_hasher.rs-0338 */ where
/* FP:stable_hasher.rs-0339 */     K: HashStable<CTX> + Eq + Hash,
/* FP:stable_hasher.rs-0340 */     V: HashStable<CTX>,
/* FP:stable_hasher.rs-0341 */     R: BuildHasher,
/* FP:stable_hasher.rs-0342 */ {
/* FP:stable_hasher.rs-0343 */     #[inline]
/* FP:stable_hasher.rs-0344 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0345 */         self.len().hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0346 */         for kv in self {
/* FP:stable_hasher.rs-0347 */             kv.hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0348 */         }
/* FP:stable_hasher.rs-0349 */     }
/* FP:stable_hasher.rs-0350 */ }
/* FP:stable_hasher.rs-0351 */ 
/* FP:stable_hasher.rs-0352 */ impl<K, R, CTX> HashStable<CTX> for indexmap::IndexSet<K, R>
/* FP:stable_hasher.rs-0353 */ where
/* FP:stable_hasher.rs-0354 */     K: HashStable<CTX> + Eq + Hash,
/* FP:stable_hasher.rs-0355 */     R: BuildHasher,
/* FP:stable_hasher.rs-0356 */ {
/* FP:stable_hasher.rs-0357 */     #[inline]
/* FP:stable_hasher.rs-0358 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0359 */         self.len().hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0360 */         for key in self {
/* FP:stable_hasher.rs-0361 */             key.hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0362 */         }
/* FP:stable_hasher.rs-0363 */     }
/* FP:stable_hasher.rs-0364 */ }
/* FP:stable_hasher.rs-0365 */ 
/* FP:stable_hasher.rs-0366 */ impl<A, const N: usize, CTX> HashStable<CTX> for SmallVec<[A; N]>
/* FP:stable_hasher.rs-0367 */ where
/* FP:stable_hasher.rs-0368 */     A: HashStable<CTX>,
/* FP:stable_hasher.rs-0369 */ {
/* FP:stable_hasher.rs-0370 */     #[inline]
/* FP:stable_hasher.rs-0371 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0372 */         self[..].hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0373 */     }
/* FP:stable_hasher.rs-0374 */ }
/* FP:stable_hasher.rs-0375 */ 
/* FP:stable_hasher.rs-0376 */ impl<T: ?Sized + HashStable<CTX>, CTX> HashStable<CTX> for Box<T> {
/* FP:stable_hasher.rs-0377 */     #[inline]
/* FP:stable_hasher.rs-0378 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0379 */         (**self).hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0380 */     }
/* FP:stable_hasher.rs-0381 */ }
/* FP:stable_hasher.rs-0382 */ 
/* FP:stable_hasher.rs-0383 */ impl<T: ?Sized + HashStable<CTX>, CTX> HashStable<CTX> for ::std::rc::Rc<T> {
/* FP:stable_hasher.rs-0384 */     #[inline]
/* FP:stable_hasher.rs-0385 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0386 */         (**self).hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0387 */     }
/* FP:stable_hasher.rs-0388 */ }
/* FP:stable_hasher.rs-0389 */ 
/* FP:stable_hasher.rs-0390 */ impl<T: ?Sized + HashStable<CTX>, CTX> HashStable<CTX> for ::std::sync::Arc<T> {
/* FP:stable_hasher.rs-0391 */     #[inline]
/* FP:stable_hasher.rs-0392 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0393 */         (**self).hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0394 */     }
/* FP:stable_hasher.rs-0395 */ }
/* FP:stable_hasher.rs-0396 */ 
/* FP:stable_hasher.rs-0397 */ impl<CTX> HashStable<CTX> for str {
/* FP:stable_hasher.rs-0398 */     #[inline]
/* FP:stable_hasher.rs-0399 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0400 */         self.as_bytes().hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0401 */     }
/* FP:stable_hasher.rs-0402 */ }
/* FP:stable_hasher.rs-0403 */ 
/* FP:stable_hasher.rs-0404 */ impl StableOrd for &str {
/* FP:stable_hasher.rs-0405 */     const CAN_USE_UNSTABLE_SORT: bool = true;
/* FP:stable_hasher.rs-0406 */ 
/* FP:stable_hasher.rs-0407 */     // Encoding and decoding doesn't change the bytes of string slices
/* FP:stable_hasher.rs-0408 */     // and `Ord::cmp` depends only on those bytes.
/* FP:stable_hasher.rs-0409 */     const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: () = ();
/* FP:stable_hasher.rs-0410 */ }
/* FP:stable_hasher.rs-0411 */ 
/* FP:stable_hasher.rs-0412 */ impl<CTX> HashStable<CTX> for String {
/* FP:stable_hasher.rs-0413 */     #[inline]
/* FP:stable_hasher.rs-0414 */     fn hash_stable(&self, hcx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0415 */         self[..].hash_stable(hcx, hasher);
/* FP:stable_hasher.rs-0416 */     }
/* FP:stable_hasher.rs-0417 */ }
/* FP:stable_hasher.rs-0418 */ 
/* FP:stable_hasher.rs-0419 */ impl StableOrd for String {
/* FP:stable_hasher.rs-0420 */     const CAN_USE_UNSTABLE_SORT: bool = true;
/* FP:stable_hasher.rs-0421 */ 
/* FP:stable_hasher.rs-0422 */     // String comparison only depends on their contents and the
/* FP:stable_hasher.rs-0423 */     // contents are not changed by (de-)serialization.
/* FP:stable_hasher.rs-0424 */     const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: () = ();
/* FP:stable_hasher.rs-0425 */ }
/* FP:stable_hasher.rs-0426 */ 
/* FP:stable_hasher.rs-0427 */ impl<HCX> ToStableHashKey<HCX> for String {
/* FP:stable_hasher.rs-0428 */     type KeyType = String;
/* FP:stable_hasher.rs-0429 */     #[inline]
/* FP:stable_hasher.rs-0430 */     fn to_stable_hash_key(&self, _: &HCX) -> Self::KeyType {
/* FP:stable_hasher.rs-0431 */         self.clone()
/* FP:stable_hasher.rs-0432 */     }
/* FP:stable_hasher.rs-0433 */ }
/* FP:stable_hasher.rs-0434 */ 
/* FP:stable_hasher.rs-0435 */ impl<HCX, T1: ToStableHashKey<HCX>, T2: ToStableHashKey<HCX>> ToStableHashKey<HCX> for (T1, T2) {
/* FP:stable_hasher.rs-0436 */     type KeyType = (T1::KeyType, T2::KeyType);
/* FP:stable_hasher.rs-0437 */     #[inline]
/* FP:stable_hasher.rs-0438 */     fn to_stable_hash_key(&self, hcx: &HCX) -> Self::KeyType {
/* FP:stable_hasher.rs-0439 */         (self.0.to_stable_hash_key(hcx), self.1.to_stable_hash_key(hcx))
/* FP:stable_hasher.rs-0440 */     }
/* FP:stable_hasher.rs-0441 */ }
/* FP:stable_hasher.rs-0442 */ 
/* FP:stable_hasher.rs-0443 */ impl<CTX> HashStable<CTX> for bool {
/* FP:stable_hasher.rs-0444 */     #[inline]
/* FP:stable_hasher.rs-0445 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0446 */         (if *self { 1u8 } else { 0u8 }).hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0447 */     }
/* FP:stable_hasher.rs-0448 */ }
/* FP:stable_hasher.rs-0449 */ 
/* FP:stable_hasher.rs-0450 */ impl StableOrd for bool {
/* FP:stable_hasher.rs-0451 */     const CAN_USE_UNSTABLE_SORT: bool = true;
/* FP:stable_hasher.rs-0452 */ 
/* FP:stable_hasher.rs-0453 */     // sort order of bools is not changed by (de-)serialization.
/* FP:stable_hasher.rs-0454 */     const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: () = ();
/* FP:stable_hasher.rs-0455 */ }
/* FP:stable_hasher.rs-0456 */ 
/* FP:stable_hasher.rs-0457 */ impl<T, CTX> HashStable<CTX> for Option<T>
/* FP:stable_hasher.rs-0458 */ where
/* FP:stable_hasher.rs-0459 */     T: HashStable<CTX>,
/* FP:stable_hasher.rs-0460 */ {
/* FP:stable_hasher.rs-0461 */     #[inline]
/* FP:stable_hasher.rs-0462 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0463 */         if let Some(ref value) = *self {
/* FP:stable_hasher.rs-0464 */             1u8.hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0465 */             value.hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0466 */         } else {
/* FP:stable_hasher.rs-0467 */             0u8.hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0468 */         }
/* FP:stable_hasher.rs-0469 */     }
/* FP:stable_hasher.rs-0470 */ }
/* FP:stable_hasher.rs-0471 */ 
/* FP:stable_hasher.rs-0472 */ impl<T: StableOrd> StableOrd for Option<T> {
/* FP:stable_hasher.rs-0473 */     const CAN_USE_UNSTABLE_SORT: bool = T::CAN_USE_UNSTABLE_SORT;
/* FP:stable_hasher.rs-0474 */ 
/* FP:stable_hasher.rs-0475 */     // the Option wrapper does not add instability to comparison.
/* FP:stable_hasher.rs-0476 */     const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: () = ();
/* FP:stable_hasher.rs-0477 */ }
/* FP:stable_hasher.rs-0478 */ 
/* FP:stable_hasher.rs-0479 */ impl<T1, T2, CTX> HashStable<CTX> for Result<T1, T2>
/* FP:stable_hasher.rs-0480 */ where
/* FP:stable_hasher.rs-0481 */     T1: HashStable<CTX>,
/* FP:stable_hasher.rs-0482 */     T2: HashStable<CTX>,
/* FP:stable_hasher.rs-0483 */ {
/* FP:stable_hasher.rs-0484 */     #[inline]
/* FP:stable_hasher.rs-0485 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0486 */         mem::discriminant(self).hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0487 */         match *self {
/* FP:stable_hasher.rs-0488 */             Ok(ref x) => x.hash_stable(ctx, hasher),
/* FP:stable_hasher.rs-0489 */             Err(ref x) => x.hash_stable(ctx, hasher),
/* FP:stable_hasher.rs-0490 */         }
/* FP:stable_hasher.rs-0491 */     }
/* FP:stable_hasher.rs-0492 */ }
/* FP:stable_hasher.rs-0493 */ 
/* FP:stable_hasher.rs-0494 */ impl<'a, T, CTX> HashStable<CTX> for &'a T
/* FP:stable_hasher.rs-0495 */ where
/* FP:stable_hasher.rs-0496 */     T: HashStable<CTX> + ?Sized,
/* FP:stable_hasher.rs-0497 */ {
/* FP:stable_hasher.rs-0498 */     #[inline]
/* FP:stable_hasher.rs-0499 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0500 */         (**self).hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0501 */     }
/* FP:stable_hasher.rs-0502 */ }
/* FP:stable_hasher.rs-0503 */ 
/* FP:stable_hasher.rs-0504 */ impl<T, CTX> HashStable<CTX> for ::std::mem::Discriminant<T> {
/* FP:stable_hasher.rs-0505 */     #[inline]
/* FP:stable_hasher.rs-0506 */     fn hash_stable(&self, _: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0507 */         ::std::hash::Hash::hash(self, hasher);
/* FP:stable_hasher.rs-0508 */     }
/* FP:stable_hasher.rs-0509 */ }
/* FP:stable_hasher.rs-0510 */ 
/* FP:stable_hasher.rs-0511 */ impl<T, CTX> HashStable<CTX> for ::std::ops::RangeInclusive<T>
/* FP:stable_hasher.rs-0512 */ where
/* FP:stable_hasher.rs-0513 */     T: HashStable<CTX>,
/* FP:stable_hasher.rs-0514 */ {
/* FP:stable_hasher.rs-0515 */     #[inline]
/* FP:stable_hasher.rs-0516 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0517 */         self.start().hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0518 */         self.end().hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0519 */     }
/* FP:stable_hasher.rs-0520 */ }
/* FP:stable_hasher.rs-0521 */ 
/* FP:stable_hasher.rs-0522 */ impl<I: Idx, T, CTX> HashStable<CTX> for IndexSlice<I, T>
/* FP:stable_hasher.rs-0523 */ where
/* FP:stable_hasher.rs-0524 */     T: HashStable<CTX>,
/* FP:stable_hasher.rs-0525 */ {
/* FP:stable_hasher.rs-0526 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0527 */         self.len().hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0528 */         for v in &self.raw {
/* FP:stable_hasher.rs-0529 */             v.hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0530 */         }
/* FP:stable_hasher.rs-0531 */     }
/* FP:stable_hasher.rs-0532 */ }
/* FP:stable_hasher.rs-0533 */ 
/* FP:stable_hasher.rs-0534 */ impl<I: Idx, T, CTX> HashStable<CTX> for IndexVec<I, T>
/* FP:stable_hasher.rs-0535 */ where
/* FP:stable_hasher.rs-0536 */     T: HashStable<CTX>,
/* FP:stable_hasher.rs-0537 */ {
/* FP:stable_hasher.rs-0538 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0539 */         self.len().hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0540 */         for v in &self.raw {
/* FP:stable_hasher.rs-0541 */             v.hash_stable(ctx, hasher);
/* FP:stable_hasher.rs-0542 */         }
/* FP:stable_hasher.rs-0543 */     }
/* FP:stable_hasher.rs-0544 */ }
/* FP:stable_hasher.rs-0545 */ 
/* FP:stable_hasher.rs-0546 */ impl<I: Idx, CTX> HashStable<CTX> for DenseBitSet<I> {
/* FP:stable_hasher.rs-0547 */     fn hash_stable(&self, _ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0548 */         ::std::hash::Hash::hash(self, hasher);
/* FP:stable_hasher.rs-0549 */     }
/* FP:stable_hasher.rs-0550 */ }
/* FP:stable_hasher.rs-0551 */ 
/* FP:stable_hasher.rs-0552 */ impl<R: Idx, C: Idx, CTX> HashStable<CTX> for bit_set::BitMatrix<R, C> {
/* FP:stable_hasher.rs-0553 */     fn hash_stable(&self, _ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0554 */         ::std::hash::Hash::hash(self, hasher);
/* FP:stable_hasher.rs-0555 */     }
/* FP:stable_hasher.rs-0556 */ }
/* FP:stable_hasher.rs-0557 */ 
/* FP:stable_hasher.rs-0558 */ impl<T, CTX> HashStable<CTX> for bit_set::FiniteBitSet<T>
/* FP:stable_hasher.rs-0559 */ where
/* FP:stable_hasher.rs-0560 */     T: HashStable<CTX> + bit_set::FiniteBitSetTy,
/* FP:stable_hasher.rs-0561 */ {
/* FP:stable_hasher.rs-0562 */     fn hash_stable(&self, hcx: &mut CTX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0563 */         self.0.hash_stable(hcx, hasher);
/* FP:stable_hasher.rs-0564 */     }
/* FP:stable_hasher.rs-0565 */ }
/* FP:stable_hasher.rs-0566 */ 
/* FP:stable_hasher.rs-0567 */ impl_stable_traits_for_trivial_type!(::std::ffi::OsStr);
/* FP:stable_hasher.rs-0568 */ 
/* FP:stable_hasher.rs-0569 */ impl_stable_traits_for_trivial_type!(::std::path::Path);
/* FP:stable_hasher.rs-0570 */ impl_stable_traits_for_trivial_type!(::std::path::PathBuf);
/* FP:stable_hasher.rs-0571 */ 
/* FP:stable_hasher.rs-0572 */ // It is not safe to implement HashStable for HashSet, HashMap or any other collection type
/* FP:stable_hasher.rs-0573 */ // with unstable but observable iteration order.
/* FP:stable_hasher.rs-0574 */ // See https://github.com/rust-lang/compiler-team/issues/533 for further information.
/* FP:stable_hasher.rs-0575 */ impl<V, HCX> !HashStable<HCX> for std::collections::HashSet<V> {}
/* FP:stable_hasher.rs-0576 */ impl<K, V, HCX> !HashStable<HCX> for std::collections::HashMap<K, V> {}
/* FP:stable_hasher.rs-0577 */ 
/* FP:stable_hasher.rs-0578 */ impl<K, V, HCX> HashStable<HCX> for ::std::collections::BTreeMap<K, V>
/* FP:stable_hasher.rs-0579 */ where
/* FP:stable_hasher.rs-0580 */     K: HashStable<HCX> + StableOrd,
/* FP:stable_hasher.rs-0581 */     V: HashStable<HCX>,
/* FP:stable_hasher.rs-0582 */ {
/* FP:stable_hasher.rs-0583 */     fn hash_stable(&self, hcx: &mut HCX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0584 */         self.len().hash_stable(hcx, hasher);
/* FP:stable_hasher.rs-0585 */         for entry in self.iter() {
/* FP:stable_hasher.rs-0586 */             entry.hash_stable(hcx, hasher);
/* FP:stable_hasher.rs-0587 */         }
/* FP:stable_hasher.rs-0588 */     }
/* FP:stable_hasher.rs-0589 */ }
/* FP:stable_hasher.rs-0590 */ 
/* FP:stable_hasher.rs-0591 */ impl<K, HCX> HashStable<HCX> for ::std::collections::BTreeSet<K>
/* FP:stable_hasher.rs-0592 */ where
/* FP:stable_hasher.rs-0593 */     K: HashStable<HCX> + StableOrd,
/* FP:stable_hasher.rs-0594 */ {
/* FP:stable_hasher.rs-0595 */     fn hash_stable(&self, hcx: &mut HCX, hasher: &mut StableHasher) {
/* FP:stable_hasher.rs-0596 */         self.len().hash_stable(hcx, hasher);
/* FP:stable_hasher.rs-0597 */         for entry in self.iter() {
/* FP:stable_hasher.rs-0598 */             entry.hash_stable(hcx, hasher);
/* FP:stable_hasher.rs-0599 */         }
/* FP:stable_hasher.rs-0600 */     }
/* FP:stable_hasher.rs-0601 */ }
/* FP:stable_hasher.rs-0602 */ 
/* FP:stable_hasher.rs-0603 */ /// Controls what data we do or do not hash.
/* FP:stable_hasher.rs-0604 */ /// Whenever a `HashStable` implementation caches its
/* FP:stable_hasher.rs-0605 */ /// result, it needs to include `HashingControls` as part
/* FP:stable_hasher.rs-0606 */ /// of the key, to ensure that it does not produce an incorrect
/* FP:stable_hasher.rs-0607 */ /// result (for example, using a `Fingerprint` produced while
/* FP:stable_hasher.rs-0608 */ /// hashing `Span`s when a `Fingerprint` without `Span`s is
/* FP:stable_hasher.rs-0609 */ /// being requested)
/* FP:stable_hasher.rs-0610 */ #[derive(Clone, Hash, Eq, PartialEq, Debug)]
/* FP:stable_hasher.rs-0611 */ pub struct HashingControls {
/* FP:stable_hasher.rs-0612 */     pub hash_spans: bool,
/* FP:stable_hasher.rs-0613 */ }