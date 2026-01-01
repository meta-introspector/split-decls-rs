/* FP:base_n.rs-0001 */ // Converts unsigned integers into a string representation with some base.
/* FP:base_n.rs-0002 */ // Bases up to and including 36 can be used for case-insensitive things.
/* FP:base_n.rs-0003 */ 
/* FP:base_n.rs-0004 */ use std::{ascii, fmt};
/* FP:base_n.rs-0005 */ 
/* FP:base_n.rs-0006 */ #[cfg(test)]
/* FP:base_n.rs-0008 */ 
/* FP:base_n.rs-0009 */ pub const MAX_BASE: usize = 64;
/* FP:base_n.rs-0010 */ pub const ALPHANUMERIC_ONLY: usize = 62;
/* FP:base_n.rs-0011 */ pub const CASE_INSENSITIVE: usize = 36;
/* FP:base_n.rs-0012 */ 
/* FP:base_n.rs-0013 */ const BASE_64: [ascii::Char; MAX_BASE] = {
/* FP:base_n.rs-0014 */     let bytes = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ@$";
/* FP:base_n.rs-0015 */     let Some(ascii) = bytes.as_ascii() else { panic!() };
/* FP:base_n.rs-0016 */     *ascii
/* FP:base_n.rs-0017 */ };
/* FP:base_n.rs-0018 */ 
/* FP:base_n.rs-0019 */ pub struct BaseNString {
/* FP:base_n.rs-0020 */     start: usize,
/* FP:base_n.rs-0021 */     buf: [ascii::Char; 128],
/* FP:base_n.rs-0022 */ }
/* FP:base_n.rs-0023 */ 
/* FP:base_n.rs-0024 */ impl std::ops::Deref for BaseNString {
/* FP:base_n.rs-0025 */     type Target = str;
/* FP:base_n.rs-0026 */ 
/* FP:base_n.rs-0027 */     fn deref(&self) -> &str {
/* FP:base_n.rs-0028 */         self.buf[self.start..].as_str()
/* FP:base_n.rs-0029 */     }
/* FP:base_n.rs-0030 */ }
/* FP:base_n.rs-0031 */ 
/* FP:base_n.rs-0032 */ impl AsRef<str> for BaseNString {
/* FP:base_n.rs-0033 */     fn as_ref(&self) -> &str {
/* FP:base_n.rs-0034 */         self.buf[self.start..].as_str()
/* FP:base_n.rs-0035 */     }
/* FP:base_n.rs-0036 */ }
/* FP:base_n.rs-0037 */ 
/* FP:base_n.rs-0038 */ impl fmt::Display for BaseNString {
/* FP:base_n.rs-0039 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:base_n.rs-0040 */         f.write_str(self)
/* FP:base_n.rs-0041 */     }
/* FP:base_n.rs-0042 */ }
/* FP:base_n.rs-0043 */ 
/* FP:base_n.rs-0044 */ // This trait just lets us reserve the exact right amount of space when doing fixed-length
/* FP:base_n.rs-0045 */ // case-insensitive encoding. Add any impls you need.
/* FP:base_n.rs-0046 */ pub trait ToBaseN: Into<u128> {
/* FP:base_n.rs-0047 */     fn encoded_len(base: usize) -> usize;
/* FP:base_n.rs-0048 */ 
/* FP:base_n.rs-0049 */     fn to_base_fixed_len(self, base: usize) -> BaseNString {
/* FP:base_n.rs-0050 */         let mut encoded = self.to_base(base);
/* FP:base_n.rs-0051 */         encoded.start = encoded.buf.len() - Self::encoded_len(base);
/* FP:base_n.rs-0052 */         encoded
/* FP:base_n.rs-0053 */     }
/* FP:base_n.rs-0054 */ 
/* FP:base_n.rs-0055 */     fn to_base(self, base: usize) -> BaseNString {
/* FP:base_n.rs-0056 */         let mut output = [ascii::Char::Digit0; 128];
/* FP:base_n.rs-0057 */ 
/* FP:base_n.rs-0058 */         let mut n: u128 = self.into();
/* FP:base_n.rs-0059 */ 
/* FP:base_n.rs-0060 */         let mut index = output.len();
/* FP:base_n.rs-0061 */         loop {
/* FP:base_n.rs-0062 */             index -= 1;
/* FP:base_n.rs-0063 */             output[index] = BASE_64[(n % base as u128) as usize];
/* FP:base_n.rs-0064 */             n /= base as u128;
/* FP:base_n.rs-0065 */ 
/* FP:base_n.rs-0066 */             if n == 0 {
/* FP:base_n.rs-0067 */                 break;
/* FP:base_n.rs-0068 */             }
/* FP:base_n.rs-0069 */         }
/* FP:base_n.rs-0070 */         assert_eq!(n, 0);
/* FP:base_n.rs-0071 */ 
/* FP:base_n.rs-0072 */         BaseNString { start: index, buf: output }
/* FP:base_n.rs-0073 */     }
/* FP:base_n.rs-0074 */ }
/* FP:base_n.rs-0075 */ 
/* FP:base_n.rs-0076 */ impl ToBaseN for u128 {
/* FP:base_n.rs-0077 */     fn encoded_len(base: usize) -> usize {
/* FP:base_n.rs-0078 */         let mut max = u128::MAX;
/* FP:base_n.rs-0079 */         let mut len = 0;
/* FP:base_n.rs-0080 */         while max > 0 {
/* FP:base_n.rs-0081 */             len += 1;
/* FP:base_n.rs-0082 */             max /= base as u128;
/* FP:base_n.rs-0083 */         }
/* FP:base_n.rs-0084 */         len
/* FP:base_n.rs-0085 */     }
/* FP:base_n.rs-0086 */ }
/* FP:base_n.rs-0087 */ 
/* FP:base_n.rs-0088 */ impl ToBaseN for u64 {
/* FP:base_n.rs-0089 */     fn encoded_len(base: usize) -> usize {
/* FP:base_n.rs-0090 */         let mut max = u64::MAX;
/* FP:base_n.rs-0091 */         let mut len = 0;
/* FP:base_n.rs-0092 */         while max > 0 {
/* FP:base_n.rs-0093 */             len += 1;
/* FP:base_n.rs-0094 */             max /= base as u64;
/* FP:base_n.rs-0095 */         }
/* FP:base_n.rs-0096 */         len
/* FP:base_n.rs-0097 */     }
/* FP:base_n.rs-0098 */ }
/* FP:base_n.rs-0099 */ 
/* FP:base_n.rs-0100 */ impl ToBaseN for u32 {
/* FP:base_n.rs-0101 */     fn encoded_len(base: usize) -> usize {
/* FP:base_n.rs-0102 */         let mut max = u32::MAX;
/* FP:base_n.rs-0103 */         let mut len = 0;
/* FP:base_n.rs-0104 */         while max > 0 {
/* FP:base_n.rs-0105 */             len += 1;
/* FP:base_n.rs-0106 */             max /= base as u32;
/* FP:base_n.rs-0107 */         }
/* FP:base_n.rs-0108 */         len
/* FP:base_n.rs-0109 */     }
/* FP:base_n.rs-0110 */ }