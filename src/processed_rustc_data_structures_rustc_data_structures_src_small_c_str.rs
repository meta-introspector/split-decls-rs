/* FP:small_c_str.rs-0001 */ use std::ffi;
/* FP:small_c_str.rs-0002 */ use std::ops::Deref;
/* FP:small_c_str.rs-0003 */ 
/* FP:small_c_str.rs-0004 */ use smallvec::SmallVec;
/* FP:small_c_str.rs-0005 */ 
/* FP:small_c_str.rs-0006 */ #[cfg(test)]
/* FP:small_c_str.rs-0008 */ 
/* FP:small_c_str.rs-0009 */ const SIZE: usize = 36;
/* FP:small_c_str.rs-0010 */ 
/* FP:small_c_str.rs-0011 */ /// Like SmallVec but for C strings.
/* FP:small_c_str.rs-0012 */ #[derive(Clone)]
/* FP:small_c_str.rs-0013 */ pub struct SmallCStr {
/* FP:small_c_str.rs-0014 */     data: SmallVec<[u8; SIZE]>,
/* FP:small_c_str.rs-0015 */ }
/* FP:small_c_str.rs-0016 */ 
/* FP:small_c_str.rs-0017 */ impl SmallCStr {
/* FP:small_c_str.rs-0018 */     #[inline]
/* FP:small_c_str.rs-0019 */     pub fn new(s: &str) -> SmallCStr {
/* FP:small_c_str.rs-0020 */         let len = s.len();
/* FP:small_c_str.rs-0021 */         let len1 = len + 1;
/* FP:small_c_str.rs-0022 */         let data = if len < SIZE {
/* FP:small_c_str.rs-0023 */             let mut buf = [0; SIZE];
/* FP:small_c_str.rs-0024 */             buf[..len].copy_from_slice(s.as_bytes());
/* FP:small_c_str.rs-0025 */             SmallVec::from_buf_and_len(buf, len1)
/* FP:small_c_str.rs-0026 */         } else {
/* FP:small_c_str.rs-0027 */             let mut data = Vec::with_capacity(len1);
/* FP:small_c_str.rs-0028 */             data.extend_from_slice(s.as_bytes());
/* FP:small_c_str.rs-0029 */             data.push(0);
/* FP:small_c_str.rs-0030 */             SmallVec::from_vec(data)
/* FP:small_c_str.rs-0031 */         };
/* FP:small_c_str.rs-0032 */         if let Err(e) = ffi::CStr::from_bytes_with_nul(&data) {
/* FP:small_c_str.rs-0033 */             panic!("The string \"{s}\" cannot be converted into a CStr: {e}");
/* FP:small_c_str.rs-0034 */         }
/* FP:small_c_str.rs-0035 */         SmallCStr { data }
/* FP:small_c_str.rs-0036 */     }
/* FP:small_c_str.rs-0037 */ 
/* FP:small_c_str.rs-0038 */     #[inline]
/* FP:small_c_str.rs-0039 */     pub fn new_with_nul(s: &str) -> SmallCStr {
/* FP:small_c_str.rs-0040 */         let b = s.as_bytes();
/* FP:small_c_str.rs-0041 */         if let Err(e) = ffi::CStr::from_bytes_with_nul(b) {
/* FP:small_c_str.rs-0042 */             panic!("The string \"{s}\" cannot be converted into a CStr: {e}");
/* FP:small_c_str.rs-0043 */         }
/* FP:small_c_str.rs-0044 */         SmallCStr { data: SmallVec::from_slice(s.as_bytes()) }
/* FP:small_c_str.rs-0045 */     }
/* FP:small_c_str.rs-0046 */ 
/* FP:small_c_str.rs-0047 */     #[inline]
/* FP:small_c_str.rs-0048 */     pub fn as_c_str(&self) -> &ffi::CStr {
/* FP:small_c_str.rs-0049 */         unsafe { ffi::CStr::from_bytes_with_nul_unchecked(&self.data) }
/* FP:small_c_str.rs-0050 */     }
/* FP:small_c_str.rs-0051 */ 
/* FP:small_c_str.rs-0052 */     #[inline]
/* FP:small_c_str.rs-0053 */     pub fn len_with_nul(&self) -> usize {
/* FP:small_c_str.rs-0054 */         self.data.len()
/* FP:small_c_str.rs-0055 */     }
/* FP:small_c_str.rs-0056 */ 
/* FP:small_c_str.rs-0057 */     pub fn spilled(&self) -> bool {
/* FP:small_c_str.rs-0058 */         self.data.spilled()
/* FP:small_c_str.rs-0059 */     }
/* FP:small_c_str.rs-0060 */ }
/* FP:small_c_str.rs-0061 */ 
/* FP:small_c_str.rs-0062 */ impl Deref for SmallCStr {
/* FP:small_c_str.rs-0063 */     type Target = ffi::CStr;
/* FP:small_c_str.rs-0064 */ 
/* FP:small_c_str.rs-0065 */     #[inline]
/* FP:small_c_str.rs-0066 */     fn deref(&self) -> &ffi::CStr {
/* FP:small_c_str.rs-0067 */         self.as_c_str()
/* FP:small_c_str.rs-0068 */     }
/* FP:small_c_str.rs-0069 */ }
/* FP:small_c_str.rs-0070 */ 
/* FP:small_c_str.rs-0071 */ impl<'a> FromIterator<&'a str> for SmallCStr {
/* FP:small_c_str.rs-0072 */     fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
/* FP:small_c_str.rs-0073 */         let mut data =
/* FP:small_c_str.rs-0074 */             iter.into_iter().flat_map(|s| s.as_bytes()).copied().collect::<SmallVec<_>>();
/* FP:small_c_str.rs-0075 */         data.push(0);
/* FP:small_c_str.rs-0076 */         if let Err(e) = ffi::CStr::from_bytes_with_nul(&data) {
/* FP:small_c_str.rs-0077 */             panic!("The iterator {data:?} cannot be converted into a CStr: {e}");
/* FP:small_c_str.rs-0078 */         }
/* FP:small_c_str.rs-0079 */         Self { data }
/* FP:small_c_str.rs-0080 */     }
/* FP:small_c_str.rs-0081 */ }
/* FP:small_c_str.rs-0082 */ 
/* FP:small_c_str.rs-0083 */ impl From<&ffi::CStr> for SmallCStr {
/* FP:small_c_str.rs-0084 */     fn from(s: &ffi::CStr) -> Self {
/* FP:small_c_str.rs-0085 */         Self { data: SmallVec::from_slice(s.to_bytes_with_nul()) }
/* FP:small_c_str.rs-0086 */     }
/* FP:small_c_str.rs-0087 */ }