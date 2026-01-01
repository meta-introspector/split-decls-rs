/* FP:tagged_ptr.rs-0001 */ // This module implements tagged pointers. In order to utilize the pointer
/* FP:tagged_ptr.rs-0002 */ // packing, you must have a tag type implementing the [`Tag`] trait.
/* FP:tagged_ptr.rs-0003 */ //
/* FP:tagged_ptr.rs-0004 */ // We assert that the tag and the reference type is compatible at compile
/* FP:tagged_ptr.rs-0005 */ // time.
/* FP:tagged_ptr.rs-0006 */ 
/* FP:tagged_ptr.rs-0007 */ use std::fmt;
/* FP:tagged_ptr.rs-0008 */ use std::hash::{Hash, Hasher};
/* FP:tagged_ptr.rs-0009 */ use std::marker::PhantomData;
/* FP:tagged_ptr.rs-0010 */ use std::num::NonZero;
/* FP:tagged_ptr.rs-0011 */ use std::ops::Deref;
/* FP:tagged_ptr.rs-0012 */ use std::ptr::NonNull;
/* FP:tagged_ptr.rs-0013 */ 
/* FP:tagged_ptr.rs-0014 */ use crate::aligned::Aligned;
/* FP:tagged_ptr.rs-0015 */ use crate::stable_hasher::{HashStable, StableHasher};
/* FP:tagged_ptr.rs-0016 */ 
/* FP:tagged_ptr.rs-0017 */ /// This describes tags that the [`TaggedRef`] struct can hold.
/* FP:tagged_ptr.rs-0018 */ ///
/* FP:tagged_ptr.rs-0019 */ /// # Safety
/* FP:tagged_ptr.rs-0020 */ ///
/* FP:tagged_ptr.rs-0021 */ /// - The [`BITS`] constant must be correct.
/* FP:tagged_ptr.rs-0022 */ /// - No more than [`BITS`] least-significant bits may be set in the returned usize.
/* FP:tagged_ptr.rs-0023 */ /// - [`Eq`] and [`Hash`] must be implementable with the returned `usize` from `into_usize`.
/* FP:tagged_ptr.rs-0024 */ ///
/* FP:tagged_ptr.rs-0025 */ /// [`BITS`]: Tag::BITS
/* FP:tagged_ptr.rs-0026 */ pub unsafe trait Tag: Copy {
/* FP:tagged_ptr.rs-0027 */     /// Number of least-significant bits in the return value of [`into_usize`]
/* FP:tagged_ptr.rs-0028 */     /// which may be non-zero. In other words this is the bit width of the
/* FP:tagged_ptr.rs-0029 */     /// value.
/* FP:tagged_ptr.rs-0030 */     ///
/* FP:tagged_ptr.rs-0031 */     /// [`into_usize`]: Tag::into_usize
/* FP:tagged_ptr.rs-0032 */     const BITS: u32;
/* FP:tagged_ptr.rs-0033 */ 
/* FP:tagged_ptr.rs-0034 */     /// Turns this tag into an integer.
/* FP:tagged_ptr.rs-0035 */     ///
/* FP:tagged_ptr.rs-0036 */     /// The inverse of this function is [`from_usize`].
/* FP:tagged_ptr.rs-0037 */     ///
/* FP:tagged_ptr.rs-0038 */     /// This function guarantees that only the least-significant [`Self::BITS`]
/* FP:tagged_ptr.rs-0039 */     /// bits can be non-zero.
/* FP:tagged_ptr.rs-0040 */     ///
/* FP:tagged_ptr.rs-0041 */     /// [`from_usize`]: Tag::from_usize
/* FP:tagged_ptr.rs-0042 */     /// [`Self::BITS`]: Tag::BITS
/* FP:tagged_ptr.rs-0043 */     fn into_usize(self) -> usize;
/* FP:tagged_ptr.rs-0044 */ 
/* FP:tagged_ptr.rs-0045 */     /// Re-creates the tag from the integer returned by [`into_usize`].
/* FP:tagged_ptr.rs-0046 */     ///
/* FP:tagged_ptr.rs-0047 */     /// # Safety
/* FP:tagged_ptr.rs-0048 */     ///
/* FP:tagged_ptr.rs-0049 */     /// The passed `tag` must be returned from [`into_usize`].
/* FP:tagged_ptr.rs-0050 */     ///
/* FP:tagged_ptr.rs-0051 */     /// [`into_usize`]: Tag::into_usize
/* FP:tagged_ptr.rs-0052 */     unsafe fn from_usize(tag: usize) -> Self;
/* FP:tagged_ptr.rs-0053 */ }
/* FP:tagged_ptr.rs-0054 */ 
/* FP:tagged_ptr.rs-0055 */ /// Returns the number of bits available for use for tags in a pointer to `T`
/* FP:tagged_ptr.rs-0056 */ /// (this is based on `T`'s alignment).
/* FP:tagged_ptr.rs-0057 */ pub const fn bits_for<T: ?Sized + Aligned>() -> u32 {
/* FP:tagged_ptr.rs-0058 */     crate::aligned::align_of::<T>().as_nonzero().trailing_zeros()
/* FP:tagged_ptr.rs-0059 */ }
/* FP:tagged_ptr.rs-0060 */ 
/* FP:tagged_ptr.rs-0061 */ /// Returns the correct [`Tag::BITS`] constant for a set of tag values.
/* FP:tagged_ptr.rs-0062 */ pub const fn bits_for_tags(mut tags: &[usize]) -> u32 {
/* FP:tagged_ptr.rs-0063 */     let mut bits = 0;
/* FP:tagged_ptr.rs-0064 */ 
/* FP:tagged_ptr.rs-0065 */     while let &[tag, ref rest @ ..] = tags {
/* FP:tagged_ptr.rs-0066 */         tags = rest;
/* FP:tagged_ptr.rs-0067 */ 
/* FP:tagged_ptr.rs-0068 */         // bits required to represent `tag`,
/* FP:tagged_ptr.rs-0069 */         // position of the most significant 1
/* FP:tagged_ptr.rs-0070 */         let b = usize::BITS - tag.leading_zeros();
/* FP:tagged_ptr.rs-0071 */         if b > bits {
/* FP:tagged_ptr.rs-0072 */             bits = b;
/* FP:tagged_ptr.rs-0073 */         }
/* FP:tagged_ptr.rs-0074 */     }
/* FP:tagged_ptr.rs-0075 */ 
/* FP:tagged_ptr.rs-0076 */     bits
/* FP:tagged_ptr.rs-0077 */ }
/* FP:tagged_ptr.rs-0078 */ 
/* FP:tagged_ptr.rs-0079 */ /// A covariant [`Copy`] tagged borrow. This is essentially `{ pointer: &'a P, tag: T }` packed
/* FP:tagged_ptr.rs-0080 */ /// in a single reference.
/* FP:tagged_ptr.rs-0081 */ pub struct TaggedRef<'a, Pointee: Aligned + ?Sized, T: Tag> {
/* FP:tagged_ptr.rs-0082 */     /// This is semantically a pair of `pointer: &'a P` and `tag: T` fields,
/* FP:tagged_ptr.rs-0083 */     /// however we pack them in a single pointer, to save space.
/* FP:tagged_ptr.rs-0084 */     ///
/* FP:tagged_ptr.rs-0085 */     /// We pack the tag into the **most**-significant bits of the pointer to
/* FP:tagged_ptr.rs-0086 */     /// ease retrieval of the value. A left shift is a multiplication and
/* FP:tagged_ptr.rs-0087 */     /// those are embeddable in instruction encoding, for example:
/* FP:tagged_ptr.rs-0088 */     ///
/* FP:tagged_ptr.rs-0089 */     /// ```asm
/* FP:tagged_ptr.rs-0090 */     /// // (<https://godbolt.org/z/jqcYPWEr3>)
/* FP:tagged_ptr.rs-0091 */     /// example::shift_read3:
/* FP:tagged_ptr.rs-0092 */     ///     mov     eax, dword ptr [8*rdi]
/* FP:tagged_ptr.rs-0093 */     ///     ret
/* FP:tagged_ptr.rs-0094 */     ///
/* FP:tagged_ptr.rs-0095 */     /// example::mask_read3:
/* FP:tagged_ptr.rs-0096 */     ///     and     rdi, -8
/* FP:tagged_ptr.rs-0097 */     ///     mov     eax, dword ptr [rdi]
/* FP:tagged_ptr.rs-0098 */     ///     ret
/* FP:tagged_ptr.rs-0099 */     /// ```
/* FP:tagged_ptr.rs-0100 */     ///
/* FP:tagged_ptr.rs-0101 */     /// This is ASM outputted by rustc for reads of values behind tagged
/* FP:tagged_ptr.rs-0102 */     /// pointers for different approaches of tagging:
/* FP:tagged_ptr.rs-0103 */     /// - `shift_read3` uses `<< 3` (the tag is in the most-significant bits)
/* FP:tagged_ptr.rs-0104 */     /// - `mask_read3` uses `& !0b111` (the tag is in the least-significant bits)
/* FP:tagged_ptr.rs-0105 */     ///
/* FP:tagged_ptr.rs-0106 */     /// The shift approach thus produces less instructions and is likely faster
/* FP:tagged_ptr.rs-0107 */     /// (see <https://godbolt.org/z/Y913sMdWb>).
/* FP:tagged_ptr.rs-0108 */     ///
/* FP:tagged_ptr.rs-0109 */     /// Encoding diagram:
/* FP:tagged_ptr.rs-0110 */     /// ```text
/* FP:tagged_ptr.rs-0111 */     /// [ packed.addr                     ]
/* FP:tagged_ptr.rs-0112 */     /// [ tag ] [ pointer.addr >> T::BITS ] <-- usize::BITS - T::BITS bits
/* FP:tagged_ptr.rs-0113 */     ///    ^
/* FP:tagged_ptr.rs-0114 */     ///    |
/* FP:tagged_ptr.rs-0115 */     /// T::BITS bits
/* FP:tagged_ptr.rs-0116 */     /// ```
/* FP:tagged_ptr.rs-0117 */     ///
/* FP:tagged_ptr.rs-0118 */     /// The tag can be retrieved by `packed.addr() >> T::BITS` and the pointer
/* FP:tagged_ptr.rs-0119 */     /// can be retrieved by `packed.map_addr(|addr| addr << T::BITS)`.
/* FP:tagged_ptr.rs-0120 */     packed: NonNull<Pointee>,
/* FP:tagged_ptr.rs-0121 */     tag_pointer_ghost: PhantomData<(&'a Pointee, T)>,
/* FP:tagged_ptr.rs-0122 */ }
/* FP:tagged_ptr.rs-0123 */ 
/* FP:tagged_ptr.rs-0124 */ impl<'a, P, T> TaggedRef<'a, P, T>
/* FP:tagged_ptr.rs-0125 */ where
/* FP:tagged_ptr.rs-0126 */     P: Aligned + ?Sized,
/* FP:tagged_ptr.rs-0127 */     T: Tag,
/* FP:tagged_ptr.rs-0128 */ {
/* FP:tagged_ptr.rs-0129 */     /// Tags `pointer` with `tag`.
/* FP:tagged_ptr.rs-0130 */     ///
/* FP:tagged_ptr.rs-0131 */     /// [`TaggedRef`]: crate::tagged_ptr::TaggedRef
/* FP:tagged_ptr.rs-0132 */     #[inline]
/* FP:tagged_ptr.rs-0133 */     pub fn new(pointer: &'a P, tag: T) -> Self {
/* FP:tagged_ptr.rs-0134 */         Self { packed: Self::pack(NonNull::from(pointer), tag), tag_pointer_ghost: PhantomData }
/* FP:tagged_ptr.rs-0135 */     }
/* FP:tagged_ptr.rs-0136 */ 
/* FP:tagged_ptr.rs-0137 */     /// Retrieves the pointer.
/* FP:tagged_ptr.rs-0138 */     #[inline]
/* FP:tagged_ptr.rs-0139 */     pub fn pointer(self) -> &'a P {
/* FP:tagged_ptr.rs-0140 */         // SAFETY: pointer_raw returns the original pointer
/* FP:tagged_ptr.rs-0141 */         unsafe { self.pointer_raw().as_ref() }
/* FP:tagged_ptr.rs-0142 */     }
/* FP:tagged_ptr.rs-0143 */ 
/* FP:tagged_ptr.rs-0144 */     /// Retrieves the tag.
/* FP:tagged_ptr.rs-0145 */     #[inline]
/* FP:tagged_ptr.rs-0146 */     pub fn tag(&self) -> T {
/* FP:tagged_ptr.rs-0147 */         // Unpack the tag, according to the `self.packed` encoding scheme
/* FP:tagged_ptr.rs-0148 */         let tag = self.packed.addr().get() >> Self::TAG_BIT_SHIFT;
/* FP:tagged_ptr.rs-0149 */ 
/* FP:tagged_ptr.rs-0150 */         // Safety:
/* FP:tagged_ptr.rs-0151 */         // The shift retrieves the original value from `T::into_usize`,
/* FP:tagged_ptr.rs-0152 */         // satisfying `T::from_usize`'s preconditions.
/* FP:tagged_ptr.rs-0153 */         unsafe { T::from_usize(tag) }
/* FP:tagged_ptr.rs-0154 */     }
/* FP:tagged_ptr.rs-0155 */ 
/* FP:tagged_ptr.rs-0156 */     /// Sets the tag to a new value.
/* FP:tagged_ptr.rs-0157 */     #[inline]
/* FP:tagged_ptr.rs-0158 */     pub fn set_tag(&mut self, tag: T) {
/* FP:tagged_ptr.rs-0159 */         self.packed = Self::pack(self.pointer_raw(), tag);
/* FP:tagged_ptr.rs-0160 */     }
/* FP:tagged_ptr.rs-0161 */ 
/* FP:tagged_ptr.rs-0162 */     const TAG_BIT_SHIFT: u32 = usize::BITS - T::BITS;
/* FP:tagged_ptr.rs-0163 */     const ASSERTION: () = { assert!(T::BITS <= bits_for::<P>()) };
/* FP:tagged_ptr.rs-0164 */ 
/* FP:tagged_ptr.rs-0165 */     /// Pack pointer `ptr` with a `tag`, according to `self.packed` encoding scheme.
/* FP:tagged_ptr.rs-0166 */     #[inline]
/* FP:tagged_ptr.rs-0167 */     fn pack(ptr: NonNull<P>, tag: T) -> NonNull<P> {
/* FP:tagged_ptr.rs-0168 */         // Trigger assert!
/* FP:tagged_ptr.rs-0169 */         let () = Self::ASSERTION;
/* FP:tagged_ptr.rs-0170 */ 
/* FP:tagged_ptr.rs-0171 */         let packed_tag = tag.into_usize() << Self::TAG_BIT_SHIFT;
/* FP:tagged_ptr.rs-0172 */ 
/* FP:tagged_ptr.rs-0173 */         ptr.map_addr(|addr| {
/* FP:tagged_ptr.rs-0174 */             // Safety:
/* FP:tagged_ptr.rs-0175 */             // - The pointer is `NonNull` => it's address is `NonZero<usize>`
/* FP:tagged_ptr.rs-0176 */             // - `P::BITS` least significant bits are always zero (`Pointer` contract)
/* FP:tagged_ptr.rs-0177 */             // - `T::BITS <= P::BITS` (from `Self::ASSERTION`)
/* FP:tagged_ptr.rs-0178 */             //
/* FP:tagged_ptr.rs-0179 */             // Thus `addr >> T::BITS` is guaranteed to be non-zero.
/* FP:tagged_ptr.rs-0180 */             //
/* FP:tagged_ptr.rs-0181 */             // `{non_zero} | packed_tag` can't make the value zero.
/* FP:tagged_ptr.rs-0182 */ 
/* FP:tagged_ptr.rs-0183 */             let packed = (addr.get() >> T::BITS) | packed_tag;
/* FP:tagged_ptr.rs-0184 */             unsafe { NonZero::new_unchecked(packed) }
/* FP:tagged_ptr.rs-0185 */         })
/* FP:tagged_ptr.rs-0186 */     }
/* FP:tagged_ptr.rs-0187 */ 
/* FP:tagged_ptr.rs-0188 */     /// Retrieves the original raw pointer from `self.packed`.
/* FP:tagged_ptr.rs-0189 */     #[inline]
/* FP:tagged_ptr.rs-0190 */     pub(super) fn pointer_raw(&self) -> NonNull<P> {
/* FP:tagged_ptr.rs-0191 */         self.packed.map_addr(|addr| unsafe { NonZero::new_unchecked(addr.get() << T::BITS) })
/* FP:tagged_ptr.rs-0192 */     }
/* FP:tagged_ptr.rs-0193 */ }
/* FP:tagged_ptr.rs-0194 */ 
/* FP:tagged_ptr.rs-0195 */ impl<P, T> Copy for TaggedRef<'_, P, T>
/* FP:tagged_ptr.rs-0196 */ where
/* FP:tagged_ptr.rs-0197 */     P: Aligned + ?Sized,
/* FP:tagged_ptr.rs-0198 */     T: Tag,
/* FP:tagged_ptr.rs-0199 */ {
/* FP:tagged_ptr.rs-0200 */ }
/* FP:tagged_ptr.rs-0201 */ 
/* FP:tagged_ptr.rs-0202 */ impl<P, T> Clone for TaggedRef<'_, P, T>
/* FP:tagged_ptr.rs-0203 */ where
/* FP:tagged_ptr.rs-0204 */     P: Aligned + ?Sized,
/* FP:tagged_ptr.rs-0205 */     T: Tag,
/* FP:tagged_ptr.rs-0206 */ {
/* FP:tagged_ptr.rs-0207 */     #[inline]
/* FP:tagged_ptr.rs-0208 */     fn clone(&self) -> Self {
/* FP:tagged_ptr.rs-0209 */         *self
/* FP:tagged_ptr.rs-0210 */     }
/* FP:tagged_ptr.rs-0211 */ }
/* FP:tagged_ptr.rs-0212 */ 
/* FP:tagged_ptr.rs-0213 */ impl<P, T> Deref for TaggedRef<'_, P, T>
/* FP:tagged_ptr.rs-0214 */ where
/* FP:tagged_ptr.rs-0215 */     P: Aligned + ?Sized,
/* FP:tagged_ptr.rs-0216 */     T: Tag,
/* FP:tagged_ptr.rs-0217 */ {
/* FP:tagged_ptr.rs-0218 */     type Target = P;
/* FP:tagged_ptr.rs-0219 */ 
/* FP:tagged_ptr.rs-0220 */     #[inline]
/* FP:tagged_ptr.rs-0221 */     fn deref(&self) -> &Self::Target {
/* FP:tagged_ptr.rs-0222 */         self.pointer()
/* FP:tagged_ptr.rs-0223 */     }
/* FP:tagged_ptr.rs-0224 */ }
/* FP:tagged_ptr.rs-0225 */ 
/* FP:tagged_ptr.rs-0226 */ impl<P, T> fmt::Debug for TaggedRef<'_, P, T>
/* FP:tagged_ptr.rs-0227 */ where
/* FP:tagged_ptr.rs-0228 */     P: Aligned + fmt::Debug + ?Sized,
/* FP:tagged_ptr.rs-0229 */     T: Tag + fmt::Debug,
/* FP:tagged_ptr.rs-0230 */ {
/* FP:tagged_ptr.rs-0231 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:tagged_ptr.rs-0232 */         f.debug_struct("TaggedRef")
/* FP:tagged_ptr.rs-0233 */             .field("pointer", &self.pointer())
/* FP:tagged_ptr.rs-0234 */             .field("tag", &self.tag())
/* FP:tagged_ptr.rs-0235 */             .finish()
/* FP:tagged_ptr.rs-0236 */     }
/* FP:tagged_ptr.rs-0237 */ }
/* FP:tagged_ptr.rs-0238 */ 
/* FP:tagged_ptr.rs-0239 */ impl<P, T> PartialEq for TaggedRef<'_, P, T>
/* FP:tagged_ptr.rs-0240 */ where
/* FP:tagged_ptr.rs-0241 */     P: Aligned + ?Sized,
/* FP:tagged_ptr.rs-0242 */     T: Tag,
/* FP:tagged_ptr.rs-0243 */ {
/* FP:tagged_ptr.rs-0244 */     #[inline]
/* FP:tagged_ptr.rs-0245 */     #[allow(ambiguous_wide_pointer_comparisons)]
/* FP:tagged_ptr.rs-0246 */     fn eq(&self, other: &Self) -> bool {
/* FP:tagged_ptr.rs-0247 */         self.packed == other.packed
/* FP:tagged_ptr.rs-0248 */     }
/* FP:tagged_ptr.rs-0249 */ }
/* FP:tagged_ptr.rs-0250 */ 
/* FP:tagged_ptr.rs-0251 */ impl<P, T: Tag> Eq for TaggedRef<'_, P, T> {}
/* FP:tagged_ptr.rs-0252 */ 
/* FP:tagged_ptr.rs-0253 */ impl<P, T: Tag> Hash for TaggedRef<'_, P, T> {
/* FP:tagged_ptr.rs-0254 */     #[inline]
/* FP:tagged_ptr.rs-0255 */     fn hash<H: Hasher>(&self, state: &mut H) {
/* FP:tagged_ptr.rs-0256 */         self.packed.hash(state);
/* FP:tagged_ptr.rs-0257 */     }
/* FP:tagged_ptr.rs-0258 */ }
/* FP:tagged_ptr.rs-0259 */ 
/* FP:tagged_ptr.rs-0260 */ impl<'a, P, T, HCX> HashStable<HCX> for TaggedRef<'a, P, T>
/* FP:tagged_ptr.rs-0261 */ where
/* FP:tagged_ptr.rs-0262 */     P: HashStable<HCX> + Aligned + ?Sized,
/* FP:tagged_ptr.rs-0263 */     T: Tag + HashStable<HCX>,
/* FP:tagged_ptr.rs-0264 */ {
/* FP:tagged_ptr.rs-0265 */     fn hash_stable(&self, hcx: &mut HCX, hasher: &mut StableHasher) {
/* FP:tagged_ptr.rs-0266 */         self.pointer().hash_stable(hcx, hasher);
/* FP:tagged_ptr.rs-0267 */         self.tag().hash_stable(hcx, hasher);
/* FP:tagged_ptr.rs-0268 */     }
/* FP:tagged_ptr.rs-0269 */ }
/* FP:tagged_ptr.rs-0270 */ 
/* FP:tagged_ptr.rs-0271 */ // Safety:
/* FP:tagged_ptr.rs-0272 */ // `TaggedRef<P, T, ..>` is semantically just `{ ptr: P, tag: T }`, as such
/* FP:tagged_ptr.rs-0273 */ // it's ok to implement `Sync` as long as `P: Sync, T: Sync`
/* FP:tagged_ptr.rs-0274 */ unsafe impl<P, T> Sync for TaggedRef<'_, P, T>
/* FP:tagged_ptr.rs-0275 */ where
/* FP:tagged_ptr.rs-0276 */     P: Sync + Aligned + ?Sized,
/* FP:tagged_ptr.rs-0277 */     T: Sync + Tag,
/* FP:tagged_ptr.rs-0278 */ {
/* FP:tagged_ptr.rs-0279 */ }
/* FP:tagged_ptr.rs-0280 */ 
/* FP:tagged_ptr.rs-0281 */ // Safety:
/* FP:tagged_ptr.rs-0282 */ // `TaggedRef<P, T, ..>` is semantically just `{ ptr: P, tag: T }`, as such
/* FP:tagged_ptr.rs-0283 */ // it's ok to implement `Send` as long as `P: Send, T: Send`
/* FP:tagged_ptr.rs-0284 */ unsafe impl<P, T> Send for TaggedRef<'_, P, T>
/* FP:tagged_ptr.rs-0285 */ where
/* FP:tagged_ptr.rs-0286 */     P: Sync + Aligned + ?Sized,
/* FP:tagged_ptr.rs-0287 */     T: Send + Tag,
/* FP:tagged_ptr.rs-0288 */ {
/* FP:tagged_ptr.rs-0289 */ }
/* FP:tagged_ptr.rs-0290 */ 
/* FP:tagged_ptr.rs-0291 */ #[cfg(test)]