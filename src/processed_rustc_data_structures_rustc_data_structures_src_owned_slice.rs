/* FP:owned_slice.rs-0001 */ use std::borrow::Borrow;
/* FP:owned_slice.rs-0002 */ use std::ops::Deref;
/* FP:owned_slice.rs-0003 */ use std::sync::Arc;
/* FP:owned_slice.rs-0004 */ 
/* FP:owned_slice.rs-0005 */ /// An owned slice.
/* FP:owned_slice.rs-0006 */ ///
/* FP:owned_slice.rs-0007 */ /// This is similar to `Arc<[u8]>` but allows slicing and using anything as the
/* FP:owned_slice.rs-0008 */ /// backing buffer.
/* FP:owned_slice.rs-0009 */ ///
/* FP:owned_slice.rs-0010 */ /// See [`slice_owned`] for `OwnedSlice` construction and examples.
/* FP:owned_slice.rs-0011 */ ///
/* FP:owned_slice.rs-0012 */ /// ---------------------------------------------------------------------------
/* FP:owned_slice.rs-0013 */ ///
/* FP:owned_slice.rs-0014 */ /// This is essentially a replacement for `owning_ref` which is a lot simpler
/* FP:owned_slice.rs-0015 */ /// and even sound! 🌸
/* FP:owned_slice.rs-0016 */ #[derive(Clone)]
/* FP:owned_slice.rs-0017 */ pub struct OwnedSlice {
/* FP:owned_slice.rs-0018 */     /// This is conceptually a `&'self.owner [u8]`.
/* FP:owned_slice.rs-0019 */     bytes: *const [u8],
/* FP:owned_slice.rs-0020 */ 
/* FP:owned_slice.rs-0021 */     // +---------------------------------------+
/* FP:owned_slice.rs-0022 */     // | We expect `dead_code` lint here,      |
/* FP:owned_slice.rs-0023 */     // | because we don't want to accidentally |
/* FP:owned_slice.rs-0024 */     // | touch the owner — otherwise the owner |
/* FP:owned_slice.rs-0025 */     // | could invalidate out `bytes` pointer  |
/* FP:owned_slice.rs-0026 */     // |                                       |
/* FP:owned_slice.rs-0027 */     // | so be quiet                           |
/* FP:owned_slice.rs-0028 */     // +----+  +-------------------------------+
/* FP:owned_slice.rs-0029 */     //       \/
/* FP:owned_slice.rs-0030 */     //      ⊂(´･◡･⊂ )∘˚˳° (I am the phantom remnant of #97770)
/* FP:owned_slice.rs-0031 */     #[expect(dead_code)]
/* FP:owned_slice.rs-0032 */     owner: Arc<dyn Send + Sync>,
/* FP:owned_slice.rs-0033 */ }
/* FP:owned_slice.rs-0034 */ 
/* FP:owned_slice.rs-0035 */ /// Makes an [`OwnedSlice`] out of an `owner` and a `slicer` function.
/* FP:owned_slice.rs-0036 */ ///
/* FP:owned_slice.rs-0037 */ /// ## Examples
/* FP:owned_slice.rs-0038 */ ///
/* FP:owned_slice.rs-0039 */ /// ```rust
/* FP:owned_slice.rs-0040 */ /// # use crate::rustc_data_structures::owned_slice::{OwnedSlice, slice_owned};
/* FP:owned_slice.rs-0041 */ /// let vec = vec![1, 2, 3, 4];
/* FP:owned_slice.rs-0042 */ ///
/* FP:owned_slice.rs-0043 */ /// // Identical to slicing via `&v[1..3]` but produces an owned slice
/* FP:owned_slice.rs-0044 */ /// let slice: OwnedSlice = slice_owned(vec, |v| &v[1..3]);
/* FP:owned_slice.rs-0045 */ /// assert_eq!(&*slice, [2, 3]);
/* FP:owned_slice.rs-0046 */ /// ```
/* FP:owned_slice.rs-0047 */ ///
/* FP:owned_slice.rs-0048 */ /// ```rust
/* FP:owned_slice.rs-0049 */ /// # use crate::rustc_data_structures::owned_slice::{OwnedSlice, slice_owned};
/* FP:owned_slice.rs-0050 */ /// # use std::ops::Deref;
/* FP:owned_slice.rs-0051 */ /// let vec = vec![1, 2, 3, 4];
/* FP:owned_slice.rs-0052 */ ///
/* FP:owned_slice.rs-0053 */ /// // Identical to slicing via `&v[..]` but produces an owned slice
/* FP:owned_slice.rs-0054 */ /// let slice: OwnedSlice = slice_owned(vec, Deref::deref);
/* FP:owned_slice.rs-0055 */ /// assert_eq!(&*slice, [1, 2, 3, 4]);
/* FP:owned_slice.rs-0056 */ /// ```
/* FP:owned_slice.rs-0057 */ pub fn slice_owned<O, F>(owner: O, slicer: F) -> OwnedSlice
/* FP:owned_slice.rs-0058 */ where
/* FP:owned_slice.rs-0059 */     O: Send + Sync + 'static,
/* FP:owned_slice.rs-0060 */     F: FnOnce(&O) -> &[u8],
/* FP:owned_slice.rs-0061 */ {
/* FP:owned_slice.rs-0062 */     try_slice_owned(owner, |x| Ok::<_, !>(slicer(x))).into_ok()
/* FP:owned_slice.rs-0063 */ }
/* FP:owned_slice.rs-0064 */ 
/* FP:owned_slice.rs-0065 */ /// Makes an [`OwnedSlice`] out of an `owner` and a `slicer` function that can fail.
/* FP:owned_slice.rs-0066 */ ///
/* FP:owned_slice.rs-0067 */ /// See [`slice_owned`] for the infallible version.
/* FP:owned_slice.rs-0068 */ pub fn try_slice_owned<O, F, E>(owner: O, slicer: F) -> Result<OwnedSlice, E>
/* FP:owned_slice.rs-0069 */ where
/* FP:owned_slice.rs-0070 */     O: Send + Sync + 'static,
/* FP:owned_slice.rs-0071 */     F: FnOnce(&O) -> Result<&[u8], E>,
/* FP:owned_slice.rs-0072 */ {
/* FP:owned_slice.rs-0073 */     // We wrap the owner of the bytes in, so it doesn't move.
/* FP:owned_slice.rs-0074 */     //
/* FP:owned_slice.rs-0075 */     // Since the owner does not move and we don't access it in any way
/* FP:owned_slice.rs-0076 */     // before dropping, there is nothing that can invalidate the bytes pointer.
/* FP:owned_slice.rs-0077 */     //
/* FP:owned_slice.rs-0078 */     // Thus, "extending" the lifetime of the reference returned from `F` is fine.
/* FP:owned_slice.rs-0079 */     // We pretend that we pass it a reference that lives as long as the returned slice.
/* FP:owned_slice.rs-0080 */     //
/* FP:owned_slice.rs-0081 */     // N.B. the HRTB on the `slicer` is important — without it the caller could provide
/* FP:owned_slice.rs-0082 */     // a short lived slice, unrelated to the owner.
/* FP:owned_slice.rs-0083 */ 
/* FP:owned_slice.rs-0084 */     let owner = Arc::new(owner);
/* FP:owned_slice.rs-0085 */     let bytes = slicer(&*owner)?;
/* FP:owned_slice.rs-0086 */ 
/* FP:owned_slice.rs-0087 */     Ok(OwnedSlice { bytes, owner })
/* FP:owned_slice.rs-0088 */ }
/* FP:owned_slice.rs-0089 */ 
/* FP:owned_slice.rs-0090 */ impl OwnedSlice {
/* FP:owned_slice.rs-0091 */     /// Slice this slice by `slicer`.
/* FP:owned_slice.rs-0092 */     ///
/* FP:owned_slice.rs-0093 */     /// # Examples
/* FP:owned_slice.rs-0094 */     ///
/* FP:owned_slice.rs-0095 */     /// ```rust
/* FP:owned_slice.rs-0096 */     /// # use crate::rustc_data_structures::owned_slice::{OwnedSlice, slice_owned};
/* FP:owned_slice.rs-0097 */     /// let vec = vec![1, 2, 3, 4];
/* FP:owned_slice.rs-0098 */     ///
/* FP:owned_slice.rs-0099 */     /// // Identical to slicing via `&v[1..3]` but produces an owned slice
/* FP:owned_slice.rs-0100 */     /// let slice: OwnedSlice = slice_owned(vec, |v| &v[..]);
/* FP:owned_slice.rs-0101 */     /// assert_eq!(&*slice, [1, 2, 3, 4]);
/* FP:owned_slice.rs-0102 */     ///
/* FP:owned_slice.rs-0103 */     /// let slice = slice.slice(|slice| &slice[1..][..2]);
/* FP:owned_slice.rs-0104 */     /// assert_eq!(&*slice, [2, 3]);
/* FP:owned_slice.rs-0105 */     /// ```
/* FP:owned_slice.rs-0106 */     ///
/* FP:owned_slice.rs-0107 */     pub fn slice(self, slicer: impl FnOnce(&[u8]) -> &[u8]) -> OwnedSlice {
/* FP:owned_slice.rs-0108 */         // This is basically identical to `try_slice_owned`,
/* FP:owned_slice.rs-0109 */         // `slicer` can only return slices of its argument or some static data,
/* FP:owned_slice.rs-0110 */         // both of which are valid while `owner` is alive.
/* FP:owned_slice.rs-0111 */ 
/* FP:owned_slice.rs-0112 */         let bytes = slicer(&self);
/* FP:owned_slice.rs-0113 */         OwnedSlice { bytes, ..self }
/* FP:owned_slice.rs-0114 */     }
/* FP:owned_slice.rs-0115 */ }
/* FP:owned_slice.rs-0116 */ 
/* FP:owned_slice.rs-0117 */ impl Deref for OwnedSlice {
/* FP:owned_slice.rs-0118 */     type Target = [u8];
/* FP:owned_slice.rs-0119 */ 
/* FP:owned_slice.rs-0120 */     #[inline]
/* FP:owned_slice.rs-0121 */     fn deref(&self) -> &[u8] {
/* FP:owned_slice.rs-0122 */         // Safety:
/* FP:owned_slice.rs-0123 */         // `self.bytes` is valid per the construction in `slice_owned`
/* FP:owned_slice.rs-0124 */         // (which is the only constructor)
/* FP:owned_slice.rs-0125 */         unsafe { &*self.bytes }
/* FP:owned_slice.rs-0126 */     }
/* FP:owned_slice.rs-0127 */ }
/* FP:owned_slice.rs-0128 */ 
/* FP:owned_slice.rs-0129 */ impl Borrow<[u8]> for OwnedSlice {
/* FP:owned_slice.rs-0130 */     #[inline]
/* FP:owned_slice.rs-0131 */     fn borrow(&self) -> &[u8] {
/* FP:owned_slice.rs-0132 */         self
/* FP:owned_slice.rs-0133 */     }
/* FP:owned_slice.rs-0134 */ }
/* FP:owned_slice.rs-0135 */ 
/* FP:owned_slice.rs-0136 */ // Safety: `OwnedSlice` is conceptually `(&'self.1 [u8], Arc<dyn Send + Sync>)`, which is `Send`
/* FP:owned_slice.rs-0137 */ unsafe impl Send for OwnedSlice {}
/* FP:owned_slice.rs-0138 */ 
/* FP:owned_slice.rs-0139 */ // Safety: `OwnedSlice` is conceptually `(&'self.1 [u8], Arc<dyn Send + Sync>)`, which is `Sync`
/* FP:owned_slice.rs-0140 */ unsafe impl Sync for OwnedSlice {}
/* FP:owned_slice.rs-0141 */ 
/* FP:owned_slice.rs-0142 */ #[cfg(test)]