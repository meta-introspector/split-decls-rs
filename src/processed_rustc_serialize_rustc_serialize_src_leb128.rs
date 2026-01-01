/* FP:leb128.rs-0001 */ // This code is very hot and uses lots of arithmetic, avoid overflow checks for performance.
/* FP:leb128.rs-0002 */ // See https://github.com/rust-lang/rust/pull/119440#issuecomment-1874255727
/* FP:leb128.rs-0003 */ use crate::int_overflow::DebugStrictAdd;
/* FP:leb128.rs-0004 */ use crate::opaque::MemDecoder;
/* FP:leb128.rs-0005 */ use crate::serialize::Decoder;
/* FP:leb128.rs-0006 */ 
/* FP:leb128.rs-0007 */ /// Returns the length of the longest LEB128 encoding for `T`, assuming `T` is an integer type
/* FP:leb128.rs-0008 */ pub const fn max_leb128_len<T>() -> usize {
/* FP:leb128.rs-0009 */     // The longest LEB128 encoding for an integer uses 7 bits per byte.
/* FP:leb128.rs-0010 */     (size_of::<T>() * 8).div_ceil(7)
/* FP:leb128.rs-0011 */ }
/* FP:leb128.rs-0012 */ 
/* FP:leb128.rs-0013 */ /// Returns the length of the longest LEB128 encoding of all supported integer types.
/* FP:leb128.rs-0014 */ pub const fn largest_max_leb128_len() -> usize {
/* FP:leb128.rs-0015 */     max_leb128_len::<u128>()
/* FP:leb128.rs-0016 */ }
/* FP:leb128.rs-0017 */ 
/* FP:leb128.rs-0018 */ macro_rules! impl_write_unsigned_leb128 {
/* FP:leb128.rs-0019 */     ($fn_name:ident, $int_ty:ty) => {
/* FP:leb128.rs-0020 */         #[inline]
/* FP:leb128.rs-0021 */         pub fn $fn_name(out: &mut [u8; max_leb128_len::<$int_ty>()], mut value: $int_ty) -> usize {
/* FP:leb128.rs-0022 */             let mut i = 0;
/* FP:leb128.rs-0023 */ 
/* FP:leb128.rs-0024 */             loop {
/* FP:leb128.rs-0025 */                 if value < 0x80 {
/* FP:leb128.rs-0026 */                     unsafe {
/* FP:leb128.rs-0027 */                         *out.get_unchecked_mut(i) = value as u8;
/* FP:leb128.rs-0028 */                     }
/* FP:leb128.rs-0029 */ 
/* FP:leb128.rs-0030 */                     i = i.debug_strict_add(1);
/* FP:leb128.rs-0031 */                     break;
/* FP:leb128.rs-0032 */                 } else {
/* FP:leb128.rs-0033 */                     unsafe {
/* FP:leb128.rs-0034 */                         *out.get_unchecked_mut(i) = ((value & 0x7f) | 0x80) as u8;
/* FP:leb128.rs-0035 */                     }
/* FP:leb128.rs-0036 */ 
/* FP:leb128.rs-0037 */                     value >>= 7;
/* FP:leb128.rs-0038 */                     i = i.debug_strict_add(1);
/* FP:leb128.rs-0039 */                 }
/* FP:leb128.rs-0040 */             }
/* FP:leb128.rs-0041 */ 
/* FP:leb128.rs-0042 */             i
/* FP:leb128.rs-0043 */         }
/* FP:leb128.rs-0044 */     };
/* FP:leb128.rs-0045 */ }
/* FP:leb128.rs-0046 */ 
/* FP:leb128.rs-0047 */ impl_write_unsigned_leb128!(write_u16_leb128, u16);
/* FP:leb128.rs-0048 */ impl_write_unsigned_leb128!(write_u32_leb128, u32);
/* FP:leb128.rs-0049 */ impl_write_unsigned_leb128!(write_u64_leb128, u64);
/* FP:leb128.rs-0050 */ impl_write_unsigned_leb128!(write_u128_leb128, u128);
/* FP:leb128.rs-0051 */ impl_write_unsigned_leb128!(write_usize_leb128, usize);
/* FP:leb128.rs-0052 */ 
/* FP:leb128.rs-0053 */ macro_rules! impl_read_unsigned_leb128 {
/* FP:leb128.rs-0054 */     ($fn_name:ident, $int_ty:ty) => {
/* FP:leb128.rs-0055 */         #[inline]
/* FP:leb128.rs-0056 */         pub fn $fn_name(decoder: &mut MemDecoder<'_>) -> $int_ty {
/* FP:leb128.rs-0057 */             // The first iteration of this loop is unpeeled. This is a
/* FP:leb128.rs-0058 */             // performance win because this code is hot and integer values less
/* FP:leb128.rs-0059 */             // than 128 are very common, typically occurring 50-80% or more of
/* FP:leb128.rs-0060 */             // the time, even for u64 and u128.
/* FP:leb128.rs-0061 */             let byte = decoder.read_u8();
/* FP:leb128.rs-0062 */             if (byte & 0x80) == 0 {
/* FP:leb128.rs-0063 */                 return byte as $int_ty;
/* FP:leb128.rs-0064 */             }
/* FP:leb128.rs-0065 */             let mut result = (byte & 0x7F) as $int_ty;
/* FP:leb128.rs-0066 */             let mut shift = 7;
/* FP:leb128.rs-0067 */             loop {
/* FP:leb128.rs-0068 */                 let byte = decoder.read_u8();
/* FP:leb128.rs-0069 */                 if (byte & 0x80) == 0 {
/* FP:leb128.rs-0070 */                     result |= (byte as $int_ty) << shift;
/* FP:leb128.rs-0071 */                     return result;
/* FP:leb128.rs-0072 */                 } else {
/* FP:leb128.rs-0073 */                     result |= ((byte & 0x7F) as $int_ty) << shift;
/* FP:leb128.rs-0074 */                 }
/* FP:leb128.rs-0075 */                 shift = shift.debug_strict_add(7);
/* FP:leb128.rs-0076 */             }
/* FP:leb128.rs-0077 */         }
/* FP:leb128.rs-0078 */     };
/* FP:leb128.rs-0079 */ }
/* FP:leb128.rs-0080 */ 
/* FP:leb128.rs-0081 */ impl_read_unsigned_leb128!(read_u16_leb128, u16);
/* FP:leb128.rs-0082 */ impl_read_unsigned_leb128!(read_u32_leb128, u32);
/* FP:leb128.rs-0083 */ impl_read_unsigned_leb128!(read_u64_leb128, u64);
/* FP:leb128.rs-0084 */ impl_read_unsigned_leb128!(read_u128_leb128, u128);
/* FP:leb128.rs-0085 */ impl_read_unsigned_leb128!(read_usize_leb128, usize);
/* FP:leb128.rs-0086 */ 
/* FP:leb128.rs-0087 */ macro_rules! impl_write_signed_leb128 {
/* FP:leb128.rs-0088 */     ($fn_name:ident, $int_ty:ty) => {
/* FP:leb128.rs-0089 */         #[inline]
/* FP:leb128.rs-0090 */         pub fn $fn_name(out: &mut [u8; max_leb128_len::<$int_ty>()], mut value: $int_ty) -> usize {
/* FP:leb128.rs-0091 */             let mut i = 0;
/* FP:leb128.rs-0092 */ 
/* FP:leb128.rs-0093 */             loop {
/* FP:leb128.rs-0094 */                 let mut byte = (value as u8) & 0x7f;
/* FP:leb128.rs-0095 */                 value >>= 7;
/* FP:leb128.rs-0096 */                 let more = !(((value == 0) && ((byte & 0x40) == 0))
/* FP:leb128.rs-0097 */                     || ((value == -1) && ((byte & 0x40) != 0)));
/* FP:leb128.rs-0098 */ 
/* FP:leb128.rs-0099 */                 if more {
/* FP:leb128.rs-0100 */                     byte |= 0x80; // Mark this byte to show that more bytes will follow.
/* FP:leb128.rs-0101 */                 }
/* FP:leb128.rs-0102 */ 
/* FP:leb128.rs-0103 */                 unsafe {
/* FP:leb128.rs-0104 */                     *out.get_unchecked_mut(i) = byte;
/* FP:leb128.rs-0105 */                 }
/* FP:leb128.rs-0106 */ 
/* FP:leb128.rs-0107 */                 i = i.debug_strict_add(1);
/* FP:leb128.rs-0108 */ 
/* FP:leb128.rs-0109 */                 if !more {
/* FP:leb128.rs-0110 */                     break;
/* FP:leb128.rs-0111 */                 }
/* FP:leb128.rs-0112 */             }
/* FP:leb128.rs-0113 */ 
/* FP:leb128.rs-0114 */             i
/* FP:leb128.rs-0115 */         }
/* FP:leb128.rs-0116 */     };
/* FP:leb128.rs-0117 */ }
/* FP:leb128.rs-0118 */ 
/* FP:leb128.rs-0119 */ impl_write_signed_leb128!(write_i16_leb128, i16);
/* FP:leb128.rs-0120 */ impl_write_signed_leb128!(write_i32_leb128, i32);
/* FP:leb128.rs-0121 */ impl_write_signed_leb128!(write_i64_leb128, i64);
/* FP:leb128.rs-0122 */ impl_write_signed_leb128!(write_i128_leb128, i128);
/* FP:leb128.rs-0123 */ impl_write_signed_leb128!(write_isize_leb128, isize);
/* FP:leb128.rs-0124 */ 
/* FP:leb128.rs-0125 */ macro_rules! impl_read_signed_leb128 {
/* FP:leb128.rs-0126 */     ($fn_name:ident, $int_ty:ty) => {
/* FP:leb128.rs-0127 */         #[inline]
/* FP:leb128.rs-0128 */         pub fn $fn_name(decoder: &mut MemDecoder<'_>) -> $int_ty {
/* FP:leb128.rs-0129 */             let mut result = 0;
/* FP:leb128.rs-0130 */             let mut shift = 0;
/* FP:leb128.rs-0131 */             let mut byte;
/* FP:leb128.rs-0132 */ 
/* FP:leb128.rs-0133 */             loop {
/* FP:leb128.rs-0134 */                 byte = decoder.read_u8();
/* FP:leb128.rs-0135 */                 result |= <$int_ty>::from(byte & 0x7F) << shift;
/* FP:leb128.rs-0136 */                 shift = shift.debug_strict_add(7);
/* FP:leb128.rs-0137 */ 
/* FP:leb128.rs-0138 */                 if (byte & 0x80) == 0 {
/* FP:leb128.rs-0139 */                     break;
/* FP:leb128.rs-0140 */                 }
/* FP:leb128.rs-0141 */             }
/* FP:leb128.rs-0142 */ 
/* FP:leb128.rs-0143 */             if (shift < <$int_ty>::BITS) && ((byte & 0x40) != 0) {
/* FP:leb128.rs-0144 */                 // sign extend
/* FP:leb128.rs-0145 */                 result |= (!0 << shift);
/* FP:leb128.rs-0146 */             }
/* FP:leb128.rs-0147 */ 
/* FP:leb128.rs-0148 */             result
/* FP:leb128.rs-0149 */         }
/* FP:leb128.rs-0150 */     };
/* FP:leb128.rs-0151 */ }
/* FP:leb128.rs-0152 */ 
/* FP:leb128.rs-0153 */ impl_read_signed_leb128!(read_i16_leb128, i16);
/* FP:leb128.rs-0154 */ impl_read_signed_leb128!(read_i32_leb128, i32);
/* FP:leb128.rs-0155 */ impl_read_signed_leb128!(read_i64_leb128, i64);
/* FP:leb128.rs-0156 */ impl_read_signed_leb128!(read_i128_leb128, i128);
/* FP:leb128.rs-0157 */ impl_read_signed_leb128!(read_isize_leb128, isize);
/* FP:leb128.rs-0158 */ 
/* FP:leb128.rs-0159 */ #[cfg(test)]