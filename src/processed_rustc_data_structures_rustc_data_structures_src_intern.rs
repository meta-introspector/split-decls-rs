/* FP:intern.rs-0001 */ use std::cmp::Ordering;
/* FP:intern.rs-0002 */ use std::fmt::{self, Debug};
/* FP:intern.rs-0003 */ use std::hash::{Hash, Hasher};
/* FP:intern.rs-0004 */ use std::ops::Deref;
/* FP:intern.rs-0005 */ use std::ptr;
/* FP:intern.rs-0006 */ 
/* FP:intern.rs-0007 */ use crate::stable_hasher::{HashStable, StableHasher};
/* FP:intern.rs-0008 */ 
/* FP:intern.rs-0009 */ mod private {
/* FP:intern.rs-0010 */     #[derive(Clone, Copy, Debug)]
/* FP:intern.rs-0011 */     pub struct PrivateZst;
/* FP:intern.rs-0012 */ }
/* FP:intern.rs-0013 */ 
/* FP:intern.rs-0014 */ /// A reference to a value that is interned, and is known to be unique.
/* FP:intern.rs-0015 */ ///
/* FP:intern.rs-0016 */ /// Note that it is possible to have a `T` and a `Interned<T>` that are (or
/* FP:intern.rs-0017 */ /// refer to) equal but different values. But if you have two different
/* FP:intern.rs-0018 */ /// `Interned<T>`s, they both refer to the same value, at a single location in
/* FP:intern.rs-0019 */ /// memory. This means that equality and hashing can be done on the value's
/* FP:intern.rs-0020 */ /// address rather than the value's contents, which can improve performance.
/* FP:intern.rs-0021 */ ///
/* FP:intern.rs-0022 */ /// The `PrivateZst` field means you can pattern match with `Interned(v, _)`
/* FP:intern.rs-0023 */ /// but you can only construct a `Interned` with `new_unchecked`, and not
/* FP:intern.rs-0024 */ /// directly.
/* FP:intern.rs-0025 */ #[rustc_pass_by_value]
/* FP:intern.rs-0026 */ pub struct Interned<'a, T>(pub &'a T, pub private::PrivateZst);
/* FP:intern.rs-0027 */ 
/* FP:intern.rs-0028 */ impl<'a, T> Interned<'a, T> {
/* FP:intern.rs-0029 */     /// Create a new `Interned` value. The value referred to *must* be interned
/* FP:intern.rs-0030 */     /// and thus be unique, and it *must* remain unique in the future. This
/* FP:intern.rs-0031 */     /// function has `_unchecked` in the name but is not `unsafe`, because if
/* FP:intern.rs-0032 */     /// the uniqueness condition is violated condition it will cause incorrect
/* FP:intern.rs-0033 */     /// behaviour but will not affect memory safety.
/* FP:intern.rs-0034 */     #[inline]
/* FP:intern.rs-0035 */     pub const fn new_unchecked(t: &'a T) -> Self {
/* FP:intern.rs-0036 */         Interned(t, private::PrivateZst)
/* FP:intern.rs-0037 */     }
/* FP:intern.rs-0038 */ }
/* FP:intern.rs-0039 */ 
/* FP:intern.rs-0040 */ impl<'a, T> Clone for Interned<'a, T> {
/* FP:intern.rs-0041 */     fn clone(&self) -> Self {
/* FP:intern.rs-0042 */         *self
/* FP:intern.rs-0043 */     }
/* FP:intern.rs-0044 */ }
/* FP:intern.rs-0045 */ 
/* FP:intern.rs-0046 */ impl<'a, T> Copy for Interned<'a, T> {}
/* FP:intern.rs-0047 */ 
/* FP:intern.rs-0048 */ impl<'a, T> Deref for Interned<'a, T> {
/* FP:intern.rs-0049 */     type Target = T;
/* FP:intern.rs-0050 */ 
/* FP:intern.rs-0051 */     #[inline]
/* FP:intern.rs-0052 */     fn deref(&self) -> &T {
/* FP:intern.rs-0053 */         self.0
/* FP:intern.rs-0054 */     }
/* FP:intern.rs-0055 */ }
/* FP:intern.rs-0056 */ 
/* FP:intern.rs-0057 */ impl<'a, T> PartialEq for Interned<'a, T> {
/* FP:intern.rs-0058 */     #[inline]
/* FP:intern.rs-0059 */     fn eq(&self, other: &Self) -> bool {
/* FP:intern.rs-0060 */         // Pointer equality implies equality, due to the uniqueness constraint.
/* FP:intern.rs-0061 */         ptr::eq(self.0, other.0)
/* FP:intern.rs-0062 */     }
/* FP:intern.rs-0063 */ }
/* FP:intern.rs-0064 */ 
/* FP:intern.rs-0065 */ impl<'a, T> Eq for Interned<'a, T> {}
/* FP:intern.rs-0066 */ 
/* FP:intern.rs-0067 */ impl<'a, T: PartialOrd> PartialOrd for Interned<'a, T> {
/* FP:intern.rs-0068 */     fn partial_cmp(&self, other: &Interned<'a, T>) -> Option<Ordering> {
/* FP:intern.rs-0069 */         // Pointer equality implies equality, due to the uniqueness constraint,
/* FP:intern.rs-0070 */         // but the contents must be compared otherwise.
/* FP:intern.rs-0071 */         if ptr::eq(self.0, other.0) {
/* FP:intern.rs-0072 */             Some(Ordering::Equal)
/* FP:intern.rs-0073 */         } else {
/* FP:intern.rs-0074 */             let res = self.0.partial_cmp(other.0);
/* FP:intern.rs-0075 */             debug_assert_ne!(res, Some(Ordering::Equal));
/* FP:intern.rs-0076 */             res
/* FP:intern.rs-0077 */         }
/* FP:intern.rs-0078 */     }
/* FP:intern.rs-0079 */ }
/* FP:intern.rs-0080 */ 
/* FP:intern.rs-0081 */ impl<'a, T: Ord> Ord for Interned<'a, T> {
/* FP:intern.rs-0082 */     fn cmp(&self, other: &Interned<'a, T>) -> Ordering {
/* FP:intern.rs-0083 */         // Pointer equality implies equality, due to the uniqueness constraint,
/* FP:intern.rs-0084 */         // but the contents must be compared otherwise.
/* FP:intern.rs-0085 */         if ptr::eq(self.0, other.0) {
/* FP:intern.rs-0086 */             Ordering::Equal
/* FP:intern.rs-0087 */         } else {
/* FP:intern.rs-0088 */             let res = self.0.cmp(other.0);
/* FP:intern.rs-0089 */             debug_assert_ne!(res, Ordering::Equal);
/* FP:intern.rs-0090 */             res
/* FP:intern.rs-0091 */         }
/* FP:intern.rs-0092 */     }
/* FP:intern.rs-0093 */ }
/* FP:intern.rs-0094 */ 
/* FP:intern.rs-0095 */ impl<'a, T> Hash for Interned<'a, T>
/* FP:intern.rs-0096 */ where
/* FP:intern.rs-0097 */     T: Hash,
/* FP:intern.rs-0098 */ {
/* FP:intern.rs-0099 */     #[inline]
/* FP:intern.rs-0100 */     fn hash<H: Hasher>(&self, s: &mut H) {
/* FP:intern.rs-0101 */         // Pointer hashing is sufficient, due to the uniqueness constraint.
/* FP:intern.rs-0102 */         ptr::hash(self.0, s)
/* FP:intern.rs-0103 */     }
/* FP:intern.rs-0104 */ }
/* FP:intern.rs-0105 */ 
/* FP:intern.rs-0106 */ impl<T, CTX> HashStable<CTX> for Interned<'_, T>
/* FP:intern.rs-0107 */ where
/* FP:intern.rs-0108 */     T: HashStable<CTX>,
/* FP:intern.rs-0109 */ {
/* FP:intern.rs-0110 */     fn hash_stable(&self, hcx: &mut CTX, hasher: &mut StableHasher) {
/* FP:intern.rs-0111 */         self.0.hash_stable(hcx, hasher);
/* FP:intern.rs-0112 */     }
/* FP:intern.rs-0113 */ }
/* FP:intern.rs-0114 */ 
/* FP:intern.rs-0115 */ impl<T: Debug> Debug for Interned<'_, T> {
/* FP:intern.rs-0116 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:intern.rs-0117 */         self.0.fmt(f)
/* FP:intern.rs-0118 */     }
/* FP:intern.rs-0119 */ }
/* FP:intern.rs-0120 */ 
/* FP:intern.rs-0121 */ #[cfg(test)]