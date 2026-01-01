/* FP:analyze_source_file.rs-0001 */ use super::*;
/* FP:analyze_source_file.rs-0002 */ 
/* FP:analyze_source_file.rs-0003 */ #[cfg(test)]
/* FP:analyze_source_file.rs-0005 */ 
/* FP:analyze_source_file.rs-0006 */ /// Finds all newlines, multi-byte characters, and non-narrow characters in a
/* FP:analyze_source_file.rs-0007 */ /// SourceFile.
/* FP:analyze_source_file.rs-0008 */ ///
/* FP:analyze_source_file.rs-0009 */ /// This function will use an SSE2 enhanced implementation if hardware support
/* FP:analyze_source_file.rs-0010 */ /// is detected at runtime.
/* FP:analyze_source_file.rs-0011 */ pub(crate) fn analyze_source_file(src: &str) -> (Vec<RelativeBytePos>, Vec<MultiByteChar>) {
/* FP:analyze_source_file.rs-0012 */     let mut lines = vec![RelativeBytePos::from_u32(0)];
/* FP:analyze_source_file.rs-0013 */     let mut multi_byte_chars = vec![];
/* FP:analyze_source_file.rs-0014 */ 
/* FP:analyze_source_file.rs-0015 */     // Calls the right implementation, depending on hardware support available.
/* FP:analyze_source_file.rs-0016 */     analyze_source_file_dispatch(src, &mut lines, &mut multi_byte_chars);
/* FP:analyze_source_file.rs-0017 */ 
/* FP:analyze_source_file.rs-0018 */     // The code above optimistically registers a new line *after* each \n
/* FP:analyze_source_file.rs-0019 */     // it encounters. If that point is already outside the source_file, remove
/* FP:analyze_source_file.rs-0020 */     // it again.
/* FP:analyze_source_file.rs-0021 */     if let Some(&last_line_start) = lines.last() {
/* FP:analyze_source_file.rs-0022 */         let source_file_end = RelativeBytePos::from_usize(src.len());
/* FP:analyze_source_file.rs-0023 */         assert!(source_file_end >= last_line_start);
/* FP:analyze_source_file.rs-0024 */         if last_line_start == source_file_end {
/* FP:analyze_source_file.rs-0025 */             lines.pop();
/* FP:analyze_source_file.rs-0026 */         }
/* FP:analyze_source_file.rs-0027 */     }
/* FP:analyze_source_file.rs-0028 */ 
/* FP:analyze_source_file.rs-0029 */     (lines, multi_byte_chars)
/* FP:analyze_source_file.rs-0030 */ }
/* FP:analyze_source_file.rs-0031 */ 
/* FP:analyze_source_file.rs-0032 */ cfg_select! {
/* FP:analyze_source_file.rs-0033 */     any(target_arch = "x86", target_arch = "x86_64") => {
/* FP:analyze_source_file.rs-0034 */         fn analyze_source_file_dispatch(
/* FP:analyze_source_file.rs-0035 */             src: &str,
/* FP:analyze_source_file.rs-0036 */             lines: &mut Vec<RelativeBytePos>,
/* FP:analyze_source_file.rs-0037 */             multi_byte_chars: &mut Vec<MultiByteChar>,
/* FP:analyze_source_file.rs-0038 */         ) {
/* FP:analyze_source_file.rs-0039 */             if is_x86_feature_detected!("sse2") {
/* FP:analyze_source_file.rs-0040 */                 unsafe {
/* FP:analyze_source_file.rs-0041 */                     analyze_source_file_sse2(src, lines, multi_byte_chars);
/* FP:analyze_source_file.rs-0042 */                 }
/* FP:analyze_source_file.rs-0043 */             } else {
/* FP:analyze_source_file.rs-0044 */                 analyze_source_file_generic(
/* FP:analyze_source_file.rs-0045 */                     src,
/* FP:analyze_source_file.rs-0046 */                     src.len(),
/* FP:analyze_source_file.rs-0047 */                     RelativeBytePos::from_u32(0),
/* FP:analyze_source_file.rs-0048 */                     lines,
/* FP:analyze_source_file.rs-0049 */                     multi_byte_chars,
/* FP:analyze_source_file.rs-0050 */                 );
/* FP:analyze_source_file.rs-0051 */             }
/* FP:analyze_source_file.rs-0052 */         }
/* FP:analyze_source_file.rs-0053 */ 
/* FP:analyze_source_file.rs-0054 */         /// Checks 16 byte chunks of text at a time. If the chunk contains
/* FP:analyze_source_file.rs-0055 */         /// something other than printable ASCII characters and newlines, the
/* FP:analyze_source_file.rs-0056 */         /// function falls back to the generic implementation. Otherwise it uses
/* FP:analyze_source_file.rs-0057 */         /// SSE2 intrinsics to quickly find all newlines.
/* FP:analyze_source_file.rs-0058 */         #[target_feature(enable = "sse2")]
/* FP:analyze_source_file.rs-0059 */         unsafe fn analyze_source_file_sse2(
/* FP:analyze_source_file.rs-0060 */             src: &str,
/* FP:analyze_source_file.rs-0061 */             lines: &mut Vec<RelativeBytePos>,
/* FP:analyze_source_file.rs-0062 */             multi_byte_chars: &mut Vec<MultiByteChar>,
/* FP:analyze_source_file.rs-0063 */         ) {
/* FP:analyze_source_file.rs-0064 */             #[cfg(target_arch = "x86")]
/* FP:analyze_source_file.rs-0065 */             use std::arch::x86::*;
/* FP:analyze_source_file.rs-0066 */             #[cfg(target_arch = "x86_64")]
/* FP:analyze_source_file.rs-0067 */             use std::arch::x86_64::*;
/* FP:analyze_source_file.rs-0068 */ 
/* FP:analyze_source_file.rs-0069 */             const CHUNK_SIZE: usize = 16;
/* FP:analyze_source_file.rs-0070 */ 
/* FP:analyze_source_file.rs-0071 */             let (chunks, tail) = src.as_bytes().as_chunks::<CHUNK_SIZE>();
/* FP:analyze_source_file.rs-0072 */ 
/* FP:analyze_source_file.rs-0073 */             // This variable keeps track of where we should start decoding a
/* FP:analyze_source_file.rs-0074 */             // chunk. If a multi-byte character spans across chunk boundaries,
/* FP:analyze_source_file.rs-0075 */             // we need to skip that part in the next chunk because we already
/* FP:analyze_source_file.rs-0076 */             // handled it.
/* FP:analyze_source_file.rs-0077 */             let mut intra_chunk_offset = 0;
/* FP:analyze_source_file.rs-0078 */ 
/* FP:analyze_source_file.rs-0079 */             for (chunk_index, chunk) in chunks.iter().enumerate() {
/* FP:analyze_source_file.rs-0080 */                 // We don't know if the pointer is aligned to 16 bytes, so we
/* FP:analyze_source_file.rs-0081 */                 // use `loadu`, which supports unaligned loading.
/* FP:analyze_source_file.rs-0082 */                 let chunk = unsafe { _mm_loadu_si128(chunk.as_ptr() as *const __m128i) };
/* FP:analyze_source_file.rs-0083 */ 
/* FP:analyze_source_file.rs-0084 */                 // For character in the chunk, see if its byte value is < 0, which
/* FP:analyze_source_file.rs-0085 */                 // indicates that it's part of a UTF-8 char.
/* FP:analyze_source_file.rs-0086 */                 let multibyte_test = _mm_cmplt_epi8(chunk, _mm_set1_epi8(0));
/* FP:analyze_source_file.rs-0087 */                 // Create a bit mask from the comparison results.
/* FP:analyze_source_file.rs-0088 */                 let multibyte_mask = _mm_movemask_epi8(multibyte_test);
/* FP:analyze_source_file.rs-0089 */ 
/* FP:analyze_source_file.rs-0090 */                 // If the bit mask is all zero, we only have ASCII chars here:
/* FP:analyze_source_file.rs-0091 */                 if multibyte_mask == 0 {
/* FP:analyze_source_file.rs-0092 */                     assert!(intra_chunk_offset == 0);
/* FP:analyze_source_file.rs-0093 */ 
/* FP:analyze_source_file.rs-0094 */                     // Check for newlines in the chunk
/* FP:analyze_source_file.rs-0095 */                     let newlines_test = _mm_cmpeq_epi8(chunk, _mm_set1_epi8(b'\n' as i8));
/* FP:analyze_source_file.rs-0096 */                     let mut newlines_mask = _mm_movemask_epi8(newlines_test);
/* FP:analyze_source_file.rs-0097 */ 
/* FP:analyze_source_file.rs-0098 */                     let output_offset = RelativeBytePos::from_usize(chunk_index * CHUNK_SIZE + 1);
/* FP:analyze_source_file.rs-0099 */ 
/* FP:analyze_source_file.rs-0100 */                     while newlines_mask != 0 {
/* FP:analyze_source_file.rs-0101 */                         let index = newlines_mask.trailing_zeros();
/* FP:analyze_source_file.rs-0102 */ 
/* FP:analyze_source_file.rs-0103 */                         lines.push(RelativeBytePos(index) + output_offset);
/* FP:analyze_source_file.rs-0104 */ 
/* FP:analyze_source_file.rs-0105 */                         // Clear the bit, so we can find the next one.
/* FP:analyze_source_file.rs-0106 */                         newlines_mask &= newlines_mask - 1;
/* FP:analyze_source_file.rs-0107 */                     }
/* FP:analyze_source_file.rs-0108 */                 } else {
/* FP:analyze_source_file.rs-0109 */                     // The slow path.
/* FP:analyze_source_file.rs-0110 */                     // There are multibyte chars in here, fallback to generic decoding.
/* FP:analyze_source_file.rs-0111 */                     let scan_start = chunk_index * CHUNK_SIZE + intra_chunk_offset;
/* FP:analyze_source_file.rs-0112 */                     intra_chunk_offset = analyze_source_file_generic(
/* FP:analyze_source_file.rs-0113 */                         &src[scan_start..],
/* FP:analyze_source_file.rs-0114 */                         CHUNK_SIZE - intra_chunk_offset,
/* FP:analyze_source_file.rs-0115 */                         RelativeBytePos::from_usize(scan_start),
/* FP:analyze_source_file.rs-0116 */                         lines,
/* FP:analyze_source_file.rs-0117 */                         multi_byte_chars,
/* FP:analyze_source_file.rs-0118 */                     );
/* FP:analyze_source_file.rs-0119 */                 }
/* FP:analyze_source_file.rs-0120 */             }
/* FP:analyze_source_file.rs-0121 */ 
/* FP:analyze_source_file.rs-0122 */             // There might still be a tail left to analyze
/* FP:analyze_source_file.rs-0123 */             let tail_start = src.len() - tail.len() + intra_chunk_offset;
/* FP:analyze_source_file.rs-0124 */             if tail_start < src.len() {
/* FP:analyze_source_file.rs-0125 */                 analyze_source_file_generic(
/* FP:analyze_source_file.rs-0126 */                     &src[tail_start..],
/* FP:analyze_source_file.rs-0127 */                     src.len() - tail_start,
/* FP:analyze_source_file.rs-0128 */                     RelativeBytePos::from_usize(tail_start),
/* FP:analyze_source_file.rs-0129 */                     lines,
/* FP:analyze_source_file.rs-0130 */                     multi_byte_chars,
/* FP:analyze_source_file.rs-0131 */                 );
/* FP:analyze_source_file.rs-0132 */             }
/* FP:analyze_source_file.rs-0133 */         }
/* FP:analyze_source_file.rs-0134 */     }
/* FP:analyze_source_file.rs-0135 */     _ => {
/* FP:analyze_source_file.rs-0136 */         // The target (or compiler version) does not support SSE2 ...
/* FP:analyze_source_file.rs-0137 */         fn analyze_source_file_dispatch(
/* FP:analyze_source_file.rs-0138 */             src: &str,
/* FP:analyze_source_file.rs-0139 */             lines: &mut Vec<RelativeBytePos>,
/* FP:analyze_source_file.rs-0140 */             multi_byte_chars: &mut Vec<MultiByteChar>,
/* FP:analyze_source_file.rs-0141 */         ) {
/* FP:analyze_source_file.rs-0142 */             analyze_source_file_generic(
/* FP:analyze_source_file.rs-0143 */                 src,
/* FP:analyze_source_file.rs-0144 */                 src.len(),
/* FP:analyze_source_file.rs-0145 */                 RelativeBytePos::from_u32(0),
/* FP:analyze_source_file.rs-0146 */                 lines,
/* FP:analyze_source_file.rs-0147 */                 multi_byte_chars,
/* FP:analyze_source_file.rs-0148 */             );
/* FP:analyze_source_file.rs-0149 */         }
/* FP:analyze_source_file.rs-0150 */     }
/* FP:analyze_source_file.rs-0151 */ }
/* FP:analyze_source_file.rs-0152 */ 
/* FP:analyze_source_file.rs-0153 */ // `scan_len` determines the number of bytes in `src` to scan. Note that the
/* FP:analyze_source_file.rs-0154 */ // function can read past `scan_len` if a multi-byte character start within the
/* FP:analyze_source_file.rs-0155 */ // range but extends past it. The overflow is returned by the function.
/* FP:analyze_source_file.rs-0156 */ fn analyze_source_file_generic(
/* FP:analyze_source_file.rs-0157 */     src: &str,
/* FP:analyze_source_file.rs-0158 */     scan_len: usize,
/* FP:analyze_source_file.rs-0159 */     output_offset: RelativeBytePos,
/* FP:analyze_source_file.rs-0160 */     lines: &mut Vec<RelativeBytePos>,
/* FP:analyze_source_file.rs-0161 */     multi_byte_chars: &mut Vec<MultiByteChar>,
/* FP:analyze_source_file.rs-0162 */ ) -> usize {
/* FP:analyze_source_file.rs-0163 */     assert!(src.len() >= scan_len);
/* FP:analyze_source_file.rs-0164 */     let mut i = 0;
/* FP:analyze_source_file.rs-0165 */     let src_bytes = src.as_bytes();
/* FP:analyze_source_file.rs-0166 */ 
/* FP:analyze_source_file.rs-0167 */     while i < scan_len {
/* FP:analyze_source_file.rs-0168 */         let byte = unsafe {
/* FP:analyze_source_file.rs-0169 */             // We verified that i < scan_len <= src.len()
/* FP:analyze_source_file.rs-0170 */             *src_bytes.get_unchecked(i)
/* FP:analyze_source_file.rs-0171 */         };
/* FP:analyze_source_file.rs-0172 */ 
/* FP:analyze_source_file.rs-0173 */         // How much to advance in order to get to the next UTF-8 char in the
/* FP:analyze_source_file.rs-0174 */         // string.
/* FP:analyze_source_file.rs-0175 */         let mut char_len = 1;
/* FP:analyze_source_file.rs-0176 */ 
/* FP:analyze_source_file.rs-0177 */         if byte == b'\n' {
/* FP:analyze_source_file.rs-0178 */             let pos = RelativeBytePos::from_usize(i) + output_offset;
/* FP:analyze_source_file.rs-0179 */             lines.push(pos + RelativeBytePos(1));
/* FP:analyze_source_file.rs-0180 */         } else if byte >= 128 {
/* FP:analyze_source_file.rs-0181 */             // This is the beginning of a multibyte char. Just decode to `char`.
/* FP:analyze_source_file.rs-0182 */             let c = src[i..].chars().next().unwrap();
/* FP:analyze_source_file.rs-0183 */             char_len = c.len_utf8();
/* FP:analyze_source_file.rs-0184 */ 
/* FP:analyze_source_file.rs-0185 */             let pos = RelativeBytePos::from_usize(i) + output_offset;
/* FP:analyze_source_file.rs-0186 */             assert!((2..=4).contains(&char_len));
/* FP:analyze_source_file.rs-0187 */             let mbc = MultiByteChar { pos, bytes: char_len as u8 };
/* FP:analyze_source_file.rs-0188 */             multi_byte_chars.push(mbc);
/* FP:analyze_source_file.rs-0189 */         }
/* FP:analyze_source_file.rs-0190 */ 
/* FP:analyze_source_file.rs-0191 */         i += char_len;
/* FP:analyze_source_file.rs-0192 */     }
/* FP:analyze_source_file.rs-0193 */ 
/* FP:analyze_source_file.rs-0194 */     i - scan_len
/* FP:analyze_source_file.rs-0195 */ }