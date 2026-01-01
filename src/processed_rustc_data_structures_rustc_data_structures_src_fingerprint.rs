/* FP:fingerprint.rs-0001 */ use std::hash::{Hash, Hasher};
/* FP:fingerprint.rs-0002 */ 
/* FP:fingerprint.rs-0003 */ use rustc_hashes::Hash64;
/* FP:fingerprint.rs-0004 */ use crate::rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
/* FP:fingerprint.rs-0005 */ 
/* FP:fingerprint.rs-0006 */ use crate::stable_hasher::{FromStableHash, StableHasherHash, impl_stable_traits_for_trivial_type};
/* FP:fingerprint.rs-0007 */ 
/* FP:fingerprint.rs-0008 */ #[cfg(test)]
/* FP:fingerprint.rs-0010 */ 
/* FP:fingerprint.rs-0011 */ #[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Copy)]
/* FP:fingerprint.rs-0012 */ #[repr(C)]
/* FP:fingerprint.rs-0013 */ pub struct Fingerprint(u64, u64);
/* FP:fingerprint.rs-0014 */ 
/* FP:fingerprint.rs-0015 */ pub trait FingerprintComponent {
/* FP:fingerprint.rs-0016 */     fn as_u64(&self) -> u64;
/* FP:fingerprint.rs-0017 */ }
/* FP:fingerprint.rs-0018 */ 
/* FP:fingerprint.rs-0019 */ impl FingerprintComponent for Hash64 {
/* FP:fingerprint.rs-0020 */     #[inline]
/* FP:fingerprint.rs-0021 */     fn as_u64(&self) -> u64 {
/* FP:fingerprint.rs-0022 */         Hash64::as_u64(*self)
/* FP:fingerprint.rs-0023 */     }
/* FP:fingerprint.rs-0024 */ }
/* FP:fingerprint.rs-0025 */ 
/* FP:fingerprint.rs-0026 */ impl FingerprintComponent for u64 {
/* FP:fingerprint.rs-0027 */     #[inline]
/* FP:fingerprint.rs-0028 */     fn as_u64(&self) -> u64 {
/* FP:fingerprint.rs-0029 */         *self
/* FP:fingerprint.rs-0030 */     }
/* FP:fingerprint.rs-0031 */ }
/* FP:fingerprint.rs-0032 */ 
/* FP:fingerprint.rs-0033 */ impl Fingerprint {
/* FP:fingerprint.rs-0034 */     pub const ZERO: Fingerprint = Fingerprint(0, 0);
/* FP:fingerprint.rs-0035 */ 
/* FP:fingerprint.rs-0036 */     #[inline]
/* FP:fingerprint.rs-0037 */     pub fn new<A, B>(_0: A, _1: B) -> Fingerprint
/* FP:fingerprint.rs-0038 */     where
/* FP:fingerprint.rs-0039 */         A: FingerprintComponent,
/* FP:fingerprint.rs-0040 */         B: FingerprintComponent,
/* FP:fingerprint.rs-0041 */     {
/* FP:fingerprint.rs-0042 */         Fingerprint(_0.as_u64(), _1.as_u64())
/* FP:fingerprint.rs-0043 */     }
/* FP:fingerprint.rs-0044 */ 
/* FP:fingerprint.rs-0045 */     #[inline]
/* FP:fingerprint.rs-0046 */     pub fn to_smaller_hash(&self) -> Hash64 {
/* FP:fingerprint.rs-0047 */         // Even though both halves of the fingerprint are expected to be good
/* FP:fingerprint.rs-0048 */         // quality hash values, let's still combine the two values because the
/* FP:fingerprint.rs-0049 */         // Fingerprints in DefPathHash have the StableCrateId portion which is
/* FP:fingerprint.rs-0050 */         // the same for all DefPathHashes from the same crate. Combining the
/* FP:fingerprint.rs-0051 */         // two halves makes sure we get a good quality hash in such cases too.
/* FP:fingerprint.rs-0052 */         Hash64::new(self.0.wrapping_mul(3).wrapping_add(self.1))
/* FP:fingerprint.rs-0053 */     }
/* FP:fingerprint.rs-0054 */ 
/* FP:fingerprint.rs-0055 */     #[inline]
/* FP:fingerprint.rs-0056 */     pub fn split(&self) -> (Hash64, Hash64) {
/* FP:fingerprint.rs-0057 */         (Hash64::new(self.0), Hash64::new(self.1))
/* FP:fingerprint.rs-0058 */     }
/* FP:fingerprint.rs-0059 */ 
/* FP:fingerprint.rs-0060 */     #[inline]
/* FP:fingerprint.rs-0061 */     pub fn combine(self, other: Fingerprint) -> Fingerprint {
/* FP:fingerprint.rs-0062 */         // See https://stackoverflow.com/a/27952689 on why this function is
/* FP:fingerprint.rs-0063 */         // implemented this way.
/* FP:fingerprint.rs-0064 */         Fingerprint(
/* FP:fingerprint.rs-0065 */             self.0.wrapping_mul(3).wrapping_add(other.0),
/* FP:fingerprint.rs-0066 */             self.1.wrapping_mul(3).wrapping_add(other.1),
/* FP:fingerprint.rs-0067 */         )
/* FP:fingerprint.rs-0068 */     }
/* FP:fingerprint.rs-0069 */ 
/* FP:fingerprint.rs-0070 */     #[inline]
/* FP:fingerprint.rs-0071 */     pub(crate) fn as_u128(self) -> u128 {
/* FP:fingerprint.rs-0072 */         u128::from(self.1) << 64 | u128::from(self.0)
/* FP:fingerprint.rs-0073 */     }
/* FP:fingerprint.rs-0074 */ 
/* FP:fingerprint.rs-0075 */     // Combines two hashes in an order independent way. Make sure this is what
/* FP:fingerprint.rs-0076 */     // you want.
/* FP:fingerprint.rs-0077 */     #[inline]
/* FP:fingerprint.rs-0078 */     pub fn combine_commutative(self, other: Fingerprint) -> Fingerprint {
/* FP:fingerprint.rs-0079 */         let a = u128::from(self.1) << 64 | u128::from(self.0);
/* FP:fingerprint.rs-0080 */         let b = u128::from(other.1) << 64 | u128::from(other.0);
/* FP:fingerprint.rs-0081 */ 
/* FP:fingerprint.rs-0082 */         let c = a.wrapping_add(b);
/* FP:fingerprint.rs-0083 */ 
/* FP:fingerprint.rs-0084 */         Fingerprint(c as u64, (c >> 64) as u64)
/* FP:fingerprint.rs-0085 */     }
/* FP:fingerprint.rs-0086 */ 
/* FP:fingerprint.rs-0087 */     pub fn to_hex(&self) -> String {
/* FP:fingerprint.rs-0088 */         format!("{:x}{:x}", self.0, self.1)
/* FP:fingerprint.rs-0089 */     }
/* FP:fingerprint.rs-0090 */ 
/* FP:fingerprint.rs-0091 */     #[inline]
/* FP:fingerprint.rs-0092 */     pub fn to_le_bytes(&self) -> [u8; 16] {
/* FP:fingerprint.rs-0093 */         // This seems to optimize to the same machine code as
/* FP:fingerprint.rs-0094 */         // `unsafe { mem::transmute(*k) }`. Well done, LLVM! :)
/* FP:fingerprint.rs-0095 */         let mut result = [0u8; 16];
/* FP:fingerprint.rs-0096 */ 
/* FP:fingerprint.rs-0097 */         let first_half: &mut [u8; 8] = (&mut result[0..8]).try_into().unwrap();
/* FP:fingerprint.rs-0098 */         *first_half = self.0.to_le_bytes();
/* FP:fingerprint.rs-0099 */ 
/* FP:fingerprint.rs-0100 */         let second_half: &mut [u8; 8] = (&mut result[8..16]).try_into().unwrap();
/* FP:fingerprint.rs-0101 */         *second_half = self.1.to_le_bytes();
/* FP:fingerprint.rs-0102 */ 
/* FP:fingerprint.rs-0103 */         result
/* FP:fingerprint.rs-0104 */     }
/* FP:fingerprint.rs-0105 */ 
/* FP:fingerprint.rs-0106 */     #[inline]
/* FP:fingerprint.rs-0107 */     pub fn from_le_bytes(bytes: [u8; 16]) -> Fingerprint {
/* FP:fingerprint.rs-0108 */         Fingerprint(
/* FP:fingerprint.rs-0109 */             u64::from_le_bytes(bytes[0..8].try_into().unwrap()),
/* FP:fingerprint.rs-0110 */             u64::from_le_bytes(bytes[8..16].try_into().unwrap()),
/* FP:fingerprint.rs-0111 */         )
/* FP:fingerprint.rs-0112 */     }
/* FP:fingerprint.rs-0113 */ }
/* FP:fingerprint.rs-0114 */ 
/* FP:fingerprint.rs-0115 */ impl std::fmt::Display for Fingerprint {
/* FP:fingerprint.rs-0116 */     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
/* FP:fingerprint.rs-0117 */         write!(formatter, "{:x}-{:x}", self.0, self.1)
/* FP:fingerprint.rs-0118 */     }
/* FP:fingerprint.rs-0119 */ }
/* FP:fingerprint.rs-0120 */ 
/* FP:fingerprint.rs-0121 */ impl Hash for Fingerprint {
/* FP:fingerprint.rs-0122 */     #[inline]
/* FP:fingerprint.rs-0123 */     fn hash<H: Hasher>(&self, state: &mut H) {
/* FP:fingerprint.rs-0124 */         state.write_fingerprint(self);
/* FP:fingerprint.rs-0125 */     }
/* FP:fingerprint.rs-0126 */ }
/* FP:fingerprint.rs-0127 */ 
/* FP:fingerprint.rs-0128 */ trait FingerprintHasher {
/* FP:fingerprint.rs-0129 */     fn write_fingerprint(&mut self, fingerprint: &Fingerprint);
/* FP:fingerprint.rs-0130 */ }
/* FP:fingerprint.rs-0131 */ 
/* FP:fingerprint.rs-0132 */ impl<H: Hasher> FingerprintHasher for H {
/* FP:fingerprint.rs-0133 */     #[inline]
/* FP:fingerprint.rs-0134 */     default fn write_fingerprint(&mut self, fingerprint: &Fingerprint) {
/* FP:fingerprint.rs-0135 */         self.write_u64(fingerprint.0);
/* FP:fingerprint.rs-0136 */         self.write_u64(fingerprint.1);
/* FP:fingerprint.rs-0137 */     }
/* FP:fingerprint.rs-0138 */ }
/* FP:fingerprint.rs-0139 */ 
/* FP:fingerprint.rs-0140 */ impl FingerprintHasher for crate::unhash::Unhasher {
/* FP:fingerprint.rs-0141 */     #[inline]
/* FP:fingerprint.rs-0142 */     fn write_fingerprint(&mut self, fingerprint: &Fingerprint) {
/* FP:fingerprint.rs-0143 */         // Even though both halves of the fingerprint are expected to be good
/* FP:fingerprint.rs-0144 */         // quality hash values, let's still combine the two values because the
/* FP:fingerprint.rs-0145 */         // Fingerprints in DefPathHash have the StableCrateId portion which is
/* FP:fingerprint.rs-0146 */         // the same for all DefPathHashes from the same crate. Combining the
/* FP:fingerprint.rs-0147 */         // two halves makes sure we get a good quality hash in such cases too.
/* FP:fingerprint.rs-0148 */         //
/* FP:fingerprint.rs-0149 */         // Since `Unhasher` is used only in the context of HashMaps, it is OK
/* FP:fingerprint.rs-0150 */         // to combine the two components in an order-independent way (which is
/* FP:fingerprint.rs-0151 */         // cheaper than the more robust Fingerprint::to_smaller_hash()). For
/* FP:fingerprint.rs-0152 */         // HashMaps we don't really care if Fingerprint(x,y) and
/* FP:fingerprint.rs-0153 */         // Fingerprint(y, x) result in the same hash value. Collision
/* FP:fingerprint.rs-0154 */         // probability will still be much better than with FxHash.
/* FP:fingerprint.rs-0155 */         self.write_u64(fingerprint.0.wrapping_add(fingerprint.1));
/* FP:fingerprint.rs-0156 */     }
/* FP:fingerprint.rs-0157 */ }
/* FP:fingerprint.rs-0158 */ 
/* FP:fingerprint.rs-0159 */ impl FromStableHash for Fingerprint {
/* FP:fingerprint.rs-0160 */     type Hash = StableHasherHash;
/* FP:fingerprint.rs-0161 */ 
/* FP:fingerprint.rs-0162 */     #[inline]
/* FP:fingerprint.rs-0163 */     fn from(StableHasherHash([_0, _1]): Self::Hash) -> Self {
/* FP:fingerprint.rs-0164 */         Fingerprint(_0, _1)
/* FP:fingerprint.rs-0165 */     }
/* FP:fingerprint.rs-0166 */ }
/* FP:fingerprint.rs-0167 */ 
/* FP:fingerprint.rs-0168 */ impl_stable_traits_for_trivial_type!(Fingerprint);
/* FP:fingerprint.rs-0169 */ 
/* FP:fingerprint.rs-0170 */ impl<E: Encoder> Encodable<E> for Fingerprint {
/* FP:fingerprint.rs-0171 */     #[inline]
/* FP:fingerprint.rs-0172 */     fn encode(&self, s: &mut E) {
/* FP:fingerprint.rs-0173 */         s.emit_raw_bytes(&self.to_le_bytes());
/* FP:fingerprint.rs-0174 */     }
/* FP:fingerprint.rs-0175 */ }
/* FP:fingerprint.rs-0176 */ 
/* FP:fingerprint.rs-0177 */ impl<D: Decoder> Decodable<D> for Fingerprint {
/* FP:fingerprint.rs-0178 */     #[inline]
/* FP:fingerprint.rs-0179 */     fn decode(d: &mut D) -> Self {
/* FP:fingerprint.rs-0180 */         Fingerprint::from_le_bytes(d.read_raw_bytes(16).try_into().unwrap())
/* FP:fingerprint.rs-0181 */     }
/* FP:fingerprint.rs-0182 */ }
/* FP:fingerprint.rs-0183 */ 
/* FP:fingerprint.rs-0184 */ // `PackedFingerprint` wraps a `Fingerprint`. Its purpose is to, on certain
/* FP:fingerprint.rs-0185 */ // architectures, behave like a `Fingerprint` without alignment requirements.
/* FP:fingerprint.rs-0186 */ // This behavior is only enabled on x86 and x86_64, where the impact of
/* FP:fingerprint.rs-0187 */ // unaligned accesses is tolerable in small doses.
/* FP:fingerprint.rs-0188 */ //
/* FP:fingerprint.rs-0189 */ // This may be preferable to use in large collections of structs containing
/* FP:fingerprint.rs-0190 */ // fingerprints, as it can reduce memory consumption by preventing the padding
/* FP:fingerprint.rs-0191 */ // that the more strictly-aligned `Fingerprint` can introduce. An application of
/* FP:fingerprint.rs-0192 */ // this is in the query dependency graph, which contains a large collection of
/* FP:fingerprint.rs-0193 */ // `DepNode`s. As of this writing, the size of a `DepNode` decreases by ~30%
/* FP:fingerprint.rs-0194 */ // (from 24 bytes to 17) by using the packed representation here, which
/* FP:fingerprint.rs-0195 */ // noticeably decreases total memory usage when compiling large crates.
/* FP:fingerprint.rs-0196 */ //
/* FP:fingerprint.rs-0197 */ // The wrapped `Fingerprint` is private to reduce the chance of a client
/* FP:fingerprint.rs-0198 */ // invoking undefined behavior by taking a reference to the packed field.
/* FP:fingerprint.rs-0199 */ #[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), repr(packed))]
/* FP:fingerprint.rs-0200 */ #[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Copy, Hash)]
/* FP:fingerprint.rs-0201 */ pub struct PackedFingerprint(Fingerprint);
/* FP:fingerprint.rs-0202 */ 
/* FP:fingerprint.rs-0203 */ impl std::fmt::Display for PackedFingerprint {
/* FP:fingerprint.rs-0204 */     #[inline]
/* FP:fingerprint.rs-0205 */     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
/* FP:fingerprint.rs-0206 */         // Copy to avoid taking reference to packed field.
/* FP:fingerprint.rs-0207 */         let copy = self.0;
/* FP:fingerprint.rs-0208 */         copy.fmt(formatter)
/* FP:fingerprint.rs-0209 */     }
/* FP:fingerprint.rs-0210 */ }
/* FP:fingerprint.rs-0211 */ 
/* FP:fingerprint.rs-0212 */ impl<E: Encoder> Encodable<E> for PackedFingerprint {
/* FP:fingerprint.rs-0213 */     #[inline]
/* FP:fingerprint.rs-0214 */     fn encode(&self, s: &mut E) {
/* FP:fingerprint.rs-0215 */         // Copy to avoid taking reference to packed field.
/* FP:fingerprint.rs-0216 */         let copy = self.0;
/* FP:fingerprint.rs-0217 */         copy.encode(s);
/* FP:fingerprint.rs-0218 */     }
/* FP:fingerprint.rs-0219 */ }
/* FP:fingerprint.rs-0220 */ 
/* FP:fingerprint.rs-0221 */ impl<D: Decoder> Decodable<D> for PackedFingerprint {
/* FP:fingerprint.rs-0222 */     #[inline]
/* FP:fingerprint.rs-0223 */     fn decode(d: &mut D) -> Self {
/* FP:fingerprint.rs-0224 */         Self(Fingerprint::decode(d))
/* FP:fingerprint.rs-0225 */     }
/* FP:fingerprint.rs-0226 */ }
/* FP:fingerprint.rs-0227 */ 
/* FP:fingerprint.rs-0228 */ impl From<Fingerprint> for PackedFingerprint {
/* FP:fingerprint.rs-0229 */     #[inline]
/* FP:fingerprint.rs-0230 */     fn from(f: Fingerprint) -> PackedFingerprint {
/* FP:fingerprint.rs-0231 */         PackedFingerprint(f)
/* FP:fingerprint.rs-0232 */     }
/* FP:fingerprint.rs-0233 */ }
/* FP:fingerprint.rs-0234 */ 
/* FP:fingerprint.rs-0235 */ impl From<PackedFingerprint> for Fingerprint {
/* FP:fingerprint.rs-0236 */     #[inline]
/* FP:fingerprint.rs-0237 */     fn from(f: PackedFingerprint) -> Fingerprint {
/* FP:fingerprint.rs-0238 */         f.0
/* FP:fingerprint.rs-0239 */     }
/* FP:fingerprint.rs-0240 */ }