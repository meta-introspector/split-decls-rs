/* FP:init_mask.rs-0001 */ #[cfg(test)]
/* FP:init_mask.rs-0003 */ 
/* FP:init_mask.rs-0004 */ use std::ops::Range;
/* FP:init_mask.rs-0005 */ use std::{hash, iter};
/* FP:init_mask.rs-0006 */ 
/* FP:init_mask.rs-0007 */ use crate::rustc_abi::Size;
/* FP:init_mask.rs-0008 */ use rustc_macros::{Decodable_NoContext, Encodable_NoContext, HashStable};
/* FP:init_mask.rs-0009 */ use crate::rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
/* FP:init_mask.rs-0010 */ 
/* FP:init_mask.rs-0011 */ use super::AllocRange;
/* FP:init_mask.rs-0012 */ 
/* FP:init_mask.rs-0013 */ type Block = u64;
/* FP:init_mask.rs-0014 */ 
/* FP:init_mask.rs-0015 */ /// A bitmask where each bit refers to the byte with the same index. If the bit is `true`, the byte
/* FP:init_mask.rs-0016 */ /// is initialized. If it is `false` the byte is uninitialized.
/* FP:init_mask.rs-0017 */ /// The actual bits are only materialized when needed, and we try to keep this data lazy as long as
/* FP:init_mask.rs-0018 */ /// possible. Currently, if all the blocks have the same value, then the mask represents either a
/* FP:init_mask.rs-0019 */ /// fully initialized or fully uninitialized const allocation, so we can only store that single
/* FP:init_mask.rs-0020 */ /// value.
/* FP:init_mask.rs-0021 */ #[derive(Clone, Debug, Eq, PartialEq, Encodable_NoContext, Decodable_NoContext, Hash, HashStable)]
/* FP:init_mask.rs-0022 */ pub struct InitMask {
/* FP:init_mask.rs-0023 */     blocks: InitMaskBlocks,
/* FP:init_mask.rs-0024 */     len: Size,
/* FP:init_mask.rs-0025 */ }
/* FP:init_mask.rs-0026 */ 
/* FP:init_mask.rs-0027 */ #[derive(Clone, Debug, Eq, PartialEq, Encodable_NoContext, Decodable_NoContext, Hash, HashStable)]
/* FP:init_mask.rs-0028 */ enum InitMaskBlocks {
/* FP:init_mask.rs-0029 */     Lazy {
/* FP:init_mask.rs-0030 */         /// Whether the lazy init mask is fully initialized or uninitialized.
/* FP:init_mask.rs-0031 */         state: bool,
/* FP:init_mask.rs-0032 */     },
/* FP:init_mask.rs-0033 */     Materialized(InitMaskMaterialized),
/* FP:init_mask.rs-0034 */ }
/* FP:init_mask.rs-0035 */ 
/* FP:init_mask.rs-0036 */ impl InitMask {
/* FP:init_mask.rs-0037 */     pub fn new(size: Size, state: bool) -> Self {
/* FP:init_mask.rs-0038 */         // Blocks start lazily allocated, until we have to materialize them.
/* FP:init_mask.rs-0039 */         let blocks = InitMaskBlocks::Lazy { state };
/* FP:init_mask.rs-0040 */         InitMask { len: size, blocks }
/* FP:init_mask.rs-0041 */     }
/* FP:init_mask.rs-0042 */ 
/* FP:init_mask.rs-0043 */     /// Checks whether the `range` is entirely initialized.
/* FP:init_mask.rs-0044 */     ///
/* FP:init_mask.rs-0045 */     /// Returns `Ok(())` if it's initialized. Otherwise returns a range of byte
/* FP:init_mask.rs-0046 */     /// indexes for the first contiguous span of the uninitialized access.
/* FP:init_mask.rs-0047 */     #[inline]
/* FP:init_mask.rs-0048 */     pub fn is_range_initialized(&self, range: AllocRange) -> Result<(), AllocRange> {
/* FP:init_mask.rs-0049 */         let end = range.end();
/* FP:init_mask.rs-0050 */         if end > self.len {
/* FP:init_mask.rs-0051 */             return Err(AllocRange::from(self.len..end));
/* FP:init_mask.rs-0052 */         }
/* FP:init_mask.rs-0053 */ 
/* FP:init_mask.rs-0054 */         match self.blocks {
/* FP:init_mask.rs-0055 */             InitMaskBlocks::Lazy { state } => {
/* FP:init_mask.rs-0056 */                 // Lazily allocated blocks represent the full mask, and cover the requested range by
/* FP:init_mask.rs-0057 */                 // definition.
/* FP:init_mask.rs-0058 */                 if state { Ok(()) } else { Err(range) }
/* FP:init_mask.rs-0059 */             }
/* FP:init_mask.rs-0060 */             InitMaskBlocks::Materialized(ref blocks) => {
/* FP:init_mask.rs-0061 */                 blocks.is_range_initialized(range.start, end)
/* FP:init_mask.rs-0062 */             }
/* FP:init_mask.rs-0063 */         }
/* FP:init_mask.rs-0064 */     }
/* FP:init_mask.rs-0065 */ 
/* FP:init_mask.rs-0066 */     /// Sets a specified range to a value. If the range is out-of-bounds, the mask will grow to
/* FP:init_mask.rs-0067 */     /// accommodate it entirely.
/* FP:init_mask.rs-0068 */     pub fn set_range(&mut self, range: AllocRange, new_state: bool) {
/* FP:init_mask.rs-0069 */         let start = range.start;
/* FP:init_mask.rs-0070 */         let end = range.end();
/* FP:init_mask.rs-0071 */ 
/* FP:init_mask.rs-0072 */         let is_full_overwrite = start == Size::ZERO && end >= self.len;
/* FP:init_mask.rs-0073 */ 
/* FP:init_mask.rs-0074 */         // Optimize the cases of a full init/uninit state, while handling growth if needed.
/* FP:init_mask.rs-0075 */         match self.blocks {
/* FP:init_mask.rs-0076 */             InitMaskBlocks::Lazy { ref mut state } if is_full_overwrite => {
/* FP:init_mask.rs-0077 */                 // This is fully overwriting the mask, and we'll still have a single initialization
/* FP:init_mask.rs-0078 */                 // state: the blocks can stay lazy.
/* FP:init_mask.rs-0079 */                 *state = new_state;
/* FP:init_mask.rs-0080 */                 self.len = end;
/* FP:init_mask.rs-0081 */             }
/* FP:init_mask.rs-0082 */             InitMaskBlocks::Materialized(_) if is_full_overwrite => {
/* FP:init_mask.rs-0083 */                 // This is also fully overwriting materialized blocks with a single initialization
/* FP:init_mask.rs-0084 */                 // state: we'll have no need for these blocks anymore and can make them lazy.
/* FP:init_mask.rs-0085 */                 self.blocks = InitMaskBlocks::Lazy { state: new_state };
/* FP:init_mask.rs-0086 */                 self.len = end;
/* FP:init_mask.rs-0087 */             }
/* FP:init_mask.rs-0088 */             InitMaskBlocks::Lazy { state } if state == new_state => {
/* FP:init_mask.rs-0089 */                 // Here we're partially overwriting the mask but the initialization state doesn't
/* FP:init_mask.rs-0090 */                 // change: the blocks can stay lazy.
/* FP:init_mask.rs-0091 */                 if end > self.len {
/* FP:init_mask.rs-0092 */                     self.len = end;
/* FP:init_mask.rs-0093 */                 }
/* FP:init_mask.rs-0094 */             }
/* FP:init_mask.rs-0095 */             _ => {
/* FP:init_mask.rs-0096 */                 // Otherwise, we have a partial overwrite that can result in a mix of initialization
/* FP:init_mask.rs-0097 */                 // states, so we'll need materialized blocks.
/* FP:init_mask.rs-0098 */                 let len = self.len;
/* FP:init_mask.rs-0099 */                 let blocks = self.materialize_blocks();
/* FP:init_mask.rs-0100 */ 
/* FP:init_mask.rs-0101 */                 // There are 3 cases of interest here, if we have:
/* FP:init_mask.rs-0102 */                 //
/* FP:init_mask.rs-0103 */                 //         [--------]
/* FP:init_mask.rs-0104 */                 //         ^        ^
/* FP:init_mask.rs-0105 */                 //         0        len
/* FP:init_mask.rs-0106 */                 //
/* FP:init_mask.rs-0107 */                 // 1) the range to set can be in-bounds:
/* FP:init_mask.rs-0108 */                 //
/* FP:init_mask.rs-0109 */                 //            xxxx = [start, end]
/* FP:init_mask.rs-0110 */                 //         [--------]
/* FP:init_mask.rs-0111 */                 //         ^        ^
/* FP:init_mask.rs-0112 */                 //         0        len
/* FP:init_mask.rs-0113 */                 //
/* FP:init_mask.rs-0114 */                 // Here, we'll simply set the single `start` to `end` range.
/* FP:init_mask.rs-0115 */                 //
/* FP:init_mask.rs-0116 */                 // 2) the range to set can be partially out-of-bounds:
/* FP:init_mask.rs-0117 */                 //
/* FP:init_mask.rs-0118 */                 //                xxxx = [start, end]
/* FP:init_mask.rs-0119 */                 //         [--------]
/* FP:init_mask.rs-0120 */                 //         ^        ^
/* FP:init_mask.rs-0121 */                 //         0        len
/* FP:init_mask.rs-0122 */                 //
/* FP:init_mask.rs-0123 */                 // We have 2 subranges to handle:
/* FP:init_mask.rs-0124 */                 // - we'll set the existing `start` to `len` range.
/* FP:init_mask.rs-0125 */                 // - we'll grow and set the `len` to `end` range.
/* FP:init_mask.rs-0126 */                 //
/* FP:init_mask.rs-0127 */                 // 3) the range to set can be fully out-of-bounds:
/* FP:init_mask.rs-0128 */                 //
/* FP:init_mask.rs-0129 */                 //                   ---xxxx = [start, end]
/* FP:init_mask.rs-0130 */                 //         [--------]
/* FP:init_mask.rs-0131 */                 //         ^        ^
/* FP:init_mask.rs-0132 */                 //         0        len
/* FP:init_mask.rs-0133 */                 //
/* FP:init_mask.rs-0134 */                 // Since we're growing the mask to a single `new_state` value, we consider the gap
/* FP:init_mask.rs-0135 */                 // from `len` to `start` to be part of the range, and have a single subrange to
/* FP:init_mask.rs-0136 */                 // handle: we'll grow and set the `len` to `end` range.
/* FP:init_mask.rs-0137 */                 //
/* FP:init_mask.rs-0138 */                 // Note that we have to materialize, set blocks, and grow the mask. We could
/* FP:init_mask.rs-0139 */                 // therefore slightly optimize things in situations where these writes overlap.
/* FP:init_mask.rs-0140 */                 // However, as of writing this, growing the mask doesn't happen in practice yet, so
/* FP:init_mask.rs-0141 */                 // we don't do this micro-optimization.
/* FP:init_mask.rs-0142 */ 
/* FP:init_mask.rs-0143 */                 if end <= len {
/* FP:init_mask.rs-0144 */                     // Handle case 1.
/* FP:init_mask.rs-0145 */                     blocks.set_range_inbounds(start, end, new_state);
/* FP:init_mask.rs-0146 */                 } else {
/* FP:init_mask.rs-0147 */                     if start < len {
/* FP:init_mask.rs-0148 */                         // Handle the first subrange of case 2.
/* FP:init_mask.rs-0149 */                         blocks.set_range_inbounds(start, len, new_state);
/* FP:init_mask.rs-0150 */                     }
/* FP:init_mask.rs-0151 */ 
/* FP:init_mask.rs-0152 */                     // Handle the second subrange of case 2, and case 3.
/* FP:init_mask.rs-0153 */                     blocks.grow(len, end - len, new_state); // `Size` operation
/* FP:init_mask.rs-0154 */                     self.len = end;
/* FP:init_mask.rs-0155 */                 }
/* FP:init_mask.rs-0156 */             }
/* FP:init_mask.rs-0157 */         }
/* FP:init_mask.rs-0158 */     }
/* FP:init_mask.rs-0159 */ 
/* FP:init_mask.rs-0160 */     /// Materializes this mask's blocks when the mask is lazy.
/* FP:init_mask.rs-0161 */     #[inline]
/* FP:init_mask.rs-0162 */     fn materialize_blocks(&mut self) -> &mut InitMaskMaterialized {
/* FP:init_mask.rs-0163 */         if let InitMaskBlocks::Lazy { state } = self.blocks {
/* FP:init_mask.rs-0164 */             self.blocks = InitMaskBlocks::Materialized(InitMaskMaterialized::new(self.len, state));
/* FP:init_mask.rs-0165 */         }
/* FP:init_mask.rs-0166 */ 
/* FP:init_mask.rs-0167 */         let InitMaskBlocks::Materialized(ref mut blocks) = self.blocks else {
/* FP:init_mask.rs-0168 */             bug!("initmask blocks must be materialized here")
/* FP:init_mask.rs-0169 */         };
/* FP:init_mask.rs-0170 */         blocks
/* FP:init_mask.rs-0171 */     }
/* FP:init_mask.rs-0172 */ 
/* FP:init_mask.rs-0173 */     /// Returns the initialization state at the specified in-bounds index.
/* FP:init_mask.rs-0174 */     #[inline]
/* FP:init_mask.rs-0175 */     pub fn get(&self, idx: Size) -> bool {
/* FP:init_mask.rs-0176 */         match self.blocks {
/* FP:init_mask.rs-0177 */             InitMaskBlocks::Lazy { state } => state,
/* FP:init_mask.rs-0178 */             InitMaskBlocks::Materialized(ref blocks) => blocks.get(idx),
/* FP:init_mask.rs-0179 */         }
/* FP:init_mask.rs-0180 */     }
/* FP:init_mask.rs-0181 */ }
/* FP:init_mask.rs-0182 */ 
/* FP:init_mask.rs-0183 */ /// The actual materialized blocks of the bitmask, when we can't keep the `InitMask` lazy.
/* FP:init_mask.rs-0184 */ // Note: for performance reasons when interning, some of the fields can be partially
/* FP:init_mask.rs-0185 */ // hashed. (see the `Hash` impl below for more details), so the impl is not derived.
/* FP:init_mask.rs-0186 */ #[derive(Clone, Debug, Eq, PartialEq, HashStable)]
/* FP:init_mask.rs-0187 */ struct InitMaskMaterialized {
/* FP:init_mask.rs-0188 */     blocks: Vec<Block>,
/* FP:init_mask.rs-0189 */ }
/* FP:init_mask.rs-0190 */ 
/* FP:init_mask.rs-0191 */ // `Block` is a `u64`, but it is a bitmask not a numeric value. If we were to just derive
/* FP:init_mask.rs-0192 */ // Encodable and Decodable we would apply varint encoding to the bitmasks, which is slower
/* FP:init_mask.rs-0193 */ // and also produces more output when the high bits of each `u64` are occupied.
/* FP:init_mask.rs-0194 */ // Note: There is probably a remaining optimization for masks that do not use an entire
/* FP:init_mask.rs-0195 */ // `Block`.
/* FP:init_mask.rs-0196 */ impl<E: Encoder> Encodable<E> for InitMaskMaterialized {
/* FP:init_mask.rs-0197 */     fn encode(&self, encoder: &mut E) {
/* FP:init_mask.rs-0198 */         encoder.emit_usize(self.blocks.len());
/* FP:init_mask.rs-0199 */         for block in &self.blocks {
/* FP:init_mask.rs-0200 */             encoder.emit_raw_bytes(&block.to_le_bytes());
/* FP:init_mask.rs-0201 */         }
/* FP:init_mask.rs-0202 */     }
/* FP:init_mask.rs-0203 */ }
/* FP:init_mask.rs-0204 */ 
/* FP:init_mask.rs-0205 */ // This implementation is deliberately not derived, see the matching `Encodable` impl.
/* FP:init_mask.rs-0206 */ impl<D: Decoder> Decodable<D> for InitMaskMaterialized {
/* FP:init_mask.rs-0207 */     fn decode(decoder: &mut D) -> Self {
/* FP:init_mask.rs-0208 */         let num_blocks = decoder.read_usize();
/* FP:init_mask.rs-0209 */         let mut blocks = Vec::with_capacity(num_blocks);
/* FP:init_mask.rs-0210 */         for _ in 0..num_blocks {
/* FP:init_mask.rs-0211 */             let bytes = decoder.read_raw_bytes(8);
/* FP:init_mask.rs-0212 */             let block = u64::from_le_bytes(bytes.try_into().unwrap());
/* FP:init_mask.rs-0213 */             blocks.push(block);
/* FP:init_mask.rs-0214 */         }
/* FP:init_mask.rs-0215 */         InitMaskMaterialized { blocks }
/* FP:init_mask.rs-0216 */     }
/* FP:init_mask.rs-0217 */ }
/* FP:init_mask.rs-0218 */ 
/* FP:init_mask.rs-0219 */ // Const allocations are only hashed for interning. However, they can be large, making the hashing
/* FP:init_mask.rs-0220 */ // expensive especially since it uses `FxHash`: it's better suited to short keys, not potentially
/* FP:init_mask.rs-0221 */ // big buffers like the allocation's init mask. We can partially hash some fields when they're
/* FP:init_mask.rs-0222 */ // large.
/* FP:init_mask.rs-0223 */ impl hash::Hash for InitMaskMaterialized {
/* FP:init_mask.rs-0224 */     fn hash<H: hash::Hasher>(&self, state: &mut H) {
/* FP:init_mask.rs-0225 */         const MAX_BLOCKS_TO_HASH: usize = super::MAX_BYTES_TO_HASH / size_of::<Block>();
/* FP:init_mask.rs-0226 */         const MAX_BLOCKS_LEN: usize = super::MAX_HASHED_BUFFER_LEN / size_of::<Block>();
/* FP:init_mask.rs-0227 */ 
/* FP:init_mask.rs-0228 */         // Partially hash the `blocks` buffer when it is large. To limit collisions with common
/* FP:init_mask.rs-0229 */         // prefixes and suffixes, we hash the length and some slices of the buffer.
/* FP:init_mask.rs-0230 */         let block_count = self.blocks.len();
/* FP:init_mask.rs-0231 */         if block_count > MAX_BLOCKS_LEN {
/* FP:init_mask.rs-0232 */             // Hash the buffer's length.
/* FP:init_mask.rs-0233 */             block_count.hash(state);
/* FP:init_mask.rs-0234 */ 
/* FP:init_mask.rs-0235 */             // And its head and tail.
/* FP:init_mask.rs-0236 */             self.blocks[..MAX_BLOCKS_TO_HASH].hash(state);
/* FP:init_mask.rs-0237 */             self.blocks[block_count - MAX_BLOCKS_TO_HASH..].hash(state);
/* FP:init_mask.rs-0238 */         } else {
/* FP:init_mask.rs-0239 */             self.blocks.hash(state);
/* FP:init_mask.rs-0240 */         }
/* FP:init_mask.rs-0241 */     }
/* FP:init_mask.rs-0242 */ }
/* FP:init_mask.rs-0243 */ 
/* FP:init_mask.rs-0244 */ impl InitMaskMaterialized {
/* FP:init_mask.rs-0245 */     const BLOCK_SIZE: u64 = 64;
/* FP:init_mask.rs-0246 */ 
/* FP:init_mask.rs-0247 */     fn new(size: Size, state: bool) -> Self {
/* FP:init_mask.rs-0248 */         let mut m = InitMaskMaterialized { blocks: vec![] };
/* FP:init_mask.rs-0249 */         m.grow(Size::ZERO, size, state);
/* FP:init_mask.rs-0250 */         m
/* FP:init_mask.rs-0251 */     }
/* FP:init_mask.rs-0252 */ 
/* FP:init_mask.rs-0253 */     #[inline]
/* FP:init_mask.rs-0254 */     fn bit_index(bits: Size) -> (usize, usize) {
/* FP:init_mask.rs-0255 */         // BLOCK_SIZE is the number of bits that can fit in a `Block`.
/* FP:init_mask.rs-0256 */         // Each bit in a `Block` represents the initialization state of one byte of an allocation,
/* FP:init_mask.rs-0257 */         // so we use `.bytes()` here.
/* FP:init_mask.rs-0258 */         let bits = bits.bytes();
/* FP:init_mask.rs-0259 */         let a = bits / Self::BLOCK_SIZE;
/* FP:init_mask.rs-0260 */         let b = bits % Self::BLOCK_SIZE;
/* FP:init_mask.rs-0261 */         (usize::try_from(a).unwrap(), usize::try_from(b).unwrap())
/* FP:init_mask.rs-0262 */     }
/* FP:init_mask.rs-0263 */ 
/* FP:init_mask.rs-0264 */     #[inline]
/* FP:init_mask.rs-0265 */     fn size_from_bit_index(block: impl TryInto<u64>, bit: impl TryInto<u64>) -> Size {
/* FP:init_mask.rs-0266 */         let block = block.try_into().ok().unwrap();
/* FP:init_mask.rs-0267 */         let bit = bit.try_into().ok().unwrap();
/* FP:init_mask.rs-0268 */         Size::from_bytes(block * Self::BLOCK_SIZE + bit)
/* FP:init_mask.rs-0269 */     }
/* FP:init_mask.rs-0270 */ 
/* FP:init_mask.rs-0271 */     /// Checks whether the `range` is entirely initialized.
/* FP:init_mask.rs-0272 */     ///
/* FP:init_mask.rs-0273 */     /// Returns `Ok(())` if it's initialized. Otherwise returns a range of byte
/* FP:init_mask.rs-0274 */     /// indexes for the first contiguous span of the uninitialized access.
/* FP:init_mask.rs-0275 */     #[inline]
/* FP:init_mask.rs-0276 */     fn is_range_initialized(&self, start: Size, end: Size) -> Result<(), AllocRange> {
/* FP:init_mask.rs-0277 */         let uninit_start = self.find_bit(start, end, false);
/* FP:init_mask.rs-0278 */ 
/* FP:init_mask.rs-0279 */         match uninit_start {
/* FP:init_mask.rs-0280 */             Some(uninit_start) => {
/* FP:init_mask.rs-0281 */                 let uninit_end = self.find_bit(uninit_start, end, true).unwrap_or(end);
/* FP:init_mask.rs-0282 */                 Err(AllocRange::from(uninit_start..uninit_end))
/* FP:init_mask.rs-0283 */             }
/* FP:init_mask.rs-0284 */             None => Ok(()),
/* FP:init_mask.rs-0285 */         }
/* FP:init_mask.rs-0286 */     }
/* FP:init_mask.rs-0287 */ 
/* FP:init_mask.rs-0288 */     fn set_range_inbounds(&mut self, start: Size, end: Size, new_state: bool) {
/* FP:init_mask.rs-0289 */         let (block_a, bit_a) = Self::bit_index(start);
/* FP:init_mask.rs-0290 */         let (block_b, bit_b) = Self::bit_index(end);
/* FP:init_mask.rs-0291 */         if block_a == block_b {
/* FP:init_mask.rs-0292 */             // First set all bits except the first `bit_a`,
/* FP:init_mask.rs-0293 */             // then unset the last `64 - bit_b` bits.
/* FP:init_mask.rs-0294 */             let range = if bit_b == 0 {
/* FP:init_mask.rs-0295 */                 u64::MAX << bit_a
/* FP:init_mask.rs-0296 */             } else {
/* FP:init_mask.rs-0297 */                 (u64::MAX << bit_a) & (u64::MAX >> (64 - bit_b))
/* FP:init_mask.rs-0298 */             };
/* FP:init_mask.rs-0299 */             if new_state {
/* FP:init_mask.rs-0300 */                 self.blocks[block_a] |= range;
/* FP:init_mask.rs-0301 */             } else {
/* FP:init_mask.rs-0302 */                 self.blocks[block_a] &= !range;
/* FP:init_mask.rs-0303 */             }
/* FP:init_mask.rs-0304 */             return;
/* FP:init_mask.rs-0305 */         }
/* FP:init_mask.rs-0306 */         // across block boundaries
/* FP:init_mask.rs-0307 */         if new_state {
/* FP:init_mask.rs-0308 */             // Set `bit_a..64` to `1`.
/* FP:init_mask.rs-0309 */             self.blocks[block_a] |= u64::MAX << bit_a;
/* FP:init_mask.rs-0310 */             // Set `0..bit_b` to `1`.
/* FP:init_mask.rs-0311 */             if bit_b != 0 {
/* FP:init_mask.rs-0312 */                 self.blocks[block_b] |= u64::MAX >> (64 - bit_b);
/* FP:init_mask.rs-0313 */             }
/* FP:init_mask.rs-0314 */             // Fill in all the other blocks (much faster than one bit at a time).
/* FP:init_mask.rs-0315 */             for block in (block_a + 1)..block_b {
/* FP:init_mask.rs-0316 */                 self.blocks[block] = u64::MAX;
/* FP:init_mask.rs-0317 */             }
/* FP:init_mask.rs-0318 */         } else {
/* FP:init_mask.rs-0319 */             // Set `bit_a..64` to `0`.
/* FP:init_mask.rs-0320 */             self.blocks[block_a] &= !(u64::MAX << bit_a);
/* FP:init_mask.rs-0321 */             // Set `0..bit_b` to `0`.
/* FP:init_mask.rs-0322 */             if bit_b != 0 {
/* FP:init_mask.rs-0323 */                 self.blocks[block_b] &= !(u64::MAX >> (64 - bit_b));
/* FP:init_mask.rs-0324 */             }
/* FP:init_mask.rs-0325 */             // Fill in all the other blocks (much faster than one bit at a time).
/* FP:init_mask.rs-0326 */             for block in (block_a + 1)..block_b {
/* FP:init_mask.rs-0327 */                 self.blocks[block] = 0;
/* FP:init_mask.rs-0328 */             }
/* FP:init_mask.rs-0329 */         }
/* FP:init_mask.rs-0330 */     }
/* FP:init_mask.rs-0331 */ 
/* FP:init_mask.rs-0332 */     #[inline]
/* FP:init_mask.rs-0333 */     fn get(&self, i: Size) -> bool {
/* FP:init_mask.rs-0334 */         let (block, bit) = Self::bit_index(i);
/* FP:init_mask.rs-0335 */         (self.blocks[block] & (1 << bit)) != 0
/* FP:init_mask.rs-0336 */     }
/* FP:init_mask.rs-0337 */ 
/* FP:init_mask.rs-0338 */     fn grow(&mut self, len: Size, amount: Size, new_state: bool) {
/* FP:init_mask.rs-0339 */         if amount.bytes() == 0 {
/* FP:init_mask.rs-0340 */             return;
/* FP:init_mask.rs-0341 */         }
/* FP:init_mask.rs-0342 */         let unused_trailing_bits =
/* FP:init_mask.rs-0343 */             u64::try_from(self.blocks.len()).unwrap() * Self::BLOCK_SIZE - len.bytes();
/* FP:init_mask.rs-0344 */ 
/* FP:init_mask.rs-0345 */         // If there's not enough capacity in the currently allocated blocks, allocate some more.
/* FP:init_mask.rs-0346 */         if amount.bytes() > unused_trailing_bits {
/* FP:init_mask.rs-0347 */             let additional_blocks = amount.bytes() / Self::BLOCK_SIZE + 1;
/* FP:init_mask.rs-0348 */ 
/* FP:init_mask.rs-0349 */             // We allocate the blocks to the correct value for the requested init state, so we won't
/* FP:init_mask.rs-0350 */             // have to manually set them with another write.
/* FP:init_mask.rs-0351 */             let block = if new_state { u64::MAX } else { 0 };
/* FP:init_mask.rs-0352 */             self.blocks
/* FP:init_mask.rs-0353 */                 .extend(iter::repeat(block).take(usize::try_from(additional_blocks).unwrap()));
/* FP:init_mask.rs-0354 */         }
/* FP:init_mask.rs-0355 */ 
/* FP:init_mask.rs-0356 */         // New blocks have already been set here, so we only need to set the unused trailing bits,
/* FP:init_mask.rs-0357 */         // if any.
/* FP:init_mask.rs-0358 */         if unused_trailing_bits > 0 {
/* FP:init_mask.rs-0359 */             let in_bounds_tail = Size::from_bytes(unused_trailing_bits);
/* FP:init_mask.rs-0360 */             self.set_range_inbounds(len, len + in_bounds_tail, new_state); // `Size` operation
/* FP:init_mask.rs-0361 */         }
/* FP:init_mask.rs-0362 */     }
/* FP:init_mask.rs-0363 */ 
/* FP:init_mask.rs-0364 */     /// Returns the index of the first bit in `start..end` (end-exclusive) that is equal to is_init.
/* FP:init_mask.rs-0365 */     fn find_bit(&self, start: Size, end: Size, is_init: bool) -> Option<Size> {
/* FP:init_mask.rs-0366 */         /// A fast implementation of `find_bit`,
/* FP:init_mask.rs-0367 */         /// which skips over an entire block at a time if it's all 0s (resp. 1s),
/* FP:init_mask.rs-0368 */         /// and finds the first 1 (resp. 0) bit inside a block using `trailing_zeros` instead of a loop.
/* FP:init_mask.rs-0369 */         ///
/* FP:init_mask.rs-0370 */         /// Note that all examples below are written with 8 (instead of 64) bit blocks for simplicity,
/* FP:init_mask.rs-0371 */         /// and with the least significant bit (and lowest block) first:
/* FP:init_mask.rs-0372 */         /// ```text
/* FP:init_mask.rs-0373 */         ///        00000000|00000000
/* FP:init_mask.rs-0374 */         ///        ^      ^ ^      ^
/* FP:init_mask.rs-0375 */         /// index: 0      7 8      15
/* FP:init_mask.rs-0376 */         /// ```
/* FP:init_mask.rs-0377 */         /// Also, if not stated, assume that `is_init = true`, that is, we are searching for the first 1 bit.
/* FP:init_mask.rs-0378 */         fn find_bit_fast(
/* FP:init_mask.rs-0379 */             init_mask: &InitMaskMaterialized,
/* FP:init_mask.rs-0380 */             start: Size,
/* FP:init_mask.rs-0381 */             end: Size,
/* FP:init_mask.rs-0382 */             is_init: bool,
/* FP:init_mask.rs-0383 */         ) -> Option<Size> {
/* FP:init_mask.rs-0384 */             /// Search one block, returning the index of the first bit equal to `is_init`.
/* FP:init_mask.rs-0385 */             fn search_block(
/* FP:init_mask.rs-0386 */                 bits: Block,
/* FP:init_mask.rs-0387 */                 block: usize,
/* FP:init_mask.rs-0388 */                 start_bit: usize,
/* FP:init_mask.rs-0389 */                 is_init: bool,
/* FP:init_mask.rs-0390 */             ) -> Option<Size> {
/* FP:init_mask.rs-0391 */                 // For the following examples, assume this function was called with:
/* FP:init_mask.rs-0392 */                 //   bits = 0b00111011
/* FP:init_mask.rs-0393 */                 //   start_bit = 3
/* FP:init_mask.rs-0394 */                 //   is_init = false
/* FP:init_mask.rs-0395 */                 // Note that, for the examples in this function, the most significant bit is written first,
/* FP:init_mask.rs-0396 */                 // which is backwards compared to the comments in `find_bit`/`find_bit_fast`.
/* FP:init_mask.rs-0397 */ 
/* FP:init_mask.rs-0398 */                 // Invert bits so we're always looking for the first set bit.
/* FP:init_mask.rs-0399 */                 //        ! 0b00111011
/* FP:init_mask.rs-0400 */                 //   bits = 0b11000100
/* FP:init_mask.rs-0401 */                 let bits = if is_init { bits } else { !bits };
/* FP:init_mask.rs-0402 */                 // Mask off unused start bits.
/* FP:init_mask.rs-0403 */                 //          0b11000100
/* FP:init_mask.rs-0404 */                 //        & 0b11111000
/* FP:init_mask.rs-0405 */                 //   bits = 0b11000000
/* FP:init_mask.rs-0406 */                 let bits = bits & (!0 << start_bit);
/* FP:init_mask.rs-0407 */                 // Find set bit, if any.
/* FP:init_mask.rs-0408 */                 //   bit = trailing_zeros(0b11000000)
/* FP:init_mask.rs-0409 */                 //   bit = 6
/* FP:init_mask.rs-0410 */                 if bits == 0 {
/* FP:init_mask.rs-0411 */                     None
/* FP:init_mask.rs-0412 */                 } else {
/* FP:init_mask.rs-0413 */                     let bit = bits.trailing_zeros();
/* FP:init_mask.rs-0414 */                     Some(InitMaskMaterialized::size_from_bit_index(block, bit))
/* FP:init_mask.rs-0415 */                 }
/* FP:init_mask.rs-0416 */             }
/* FP:init_mask.rs-0417 */ 
/* FP:init_mask.rs-0418 */             if start >= end {
/* FP:init_mask.rs-0419 */                 return None;
/* FP:init_mask.rs-0420 */             }
/* FP:init_mask.rs-0421 */ 
/* FP:init_mask.rs-0422 */             // Convert `start` and `end` to block indexes and bit indexes within each block.
/* FP:init_mask.rs-0423 */             // We must convert `end` to an inclusive bound to handle block boundaries correctly.
/* FP:init_mask.rs-0424 */             //
/* FP:init_mask.rs-0425 */             // For example:
/* FP:init_mask.rs-0426 */             //
/* FP:init_mask.rs-0427 */             //   (a) 00000000|00000000    (b) 00000000|
/* FP:init_mask.rs-0428 */             //       ^~~~~~~~~~~^             ^~~~~~~~~^
/* FP:init_mask.rs-0429 */             //     start       end          start     end
/* FP:init_mask.rs-0430 */             //
/* FP:init_mask.rs-0431 */             // In both cases, the block index of `end` is 1.
/* FP:init_mask.rs-0432 */             // But we do want to search block 1 in (a), and we don't in (b).
/* FP:init_mask.rs-0433 */             //
/* FP:init_mask.rs-0434 */             // We subtract 1 from both end positions to make them inclusive:
/* FP:init_mask.rs-0435 */             //
/* FP:init_mask.rs-0436 */             //   (a) 00000000|00000000    (b) 00000000|
/* FP:init_mask.rs-0437 */             //       ^~~~~~~~~~^              ^~~~~~~^
/* FP:init_mask.rs-0438 */             //     start    end_inclusive   start end_inclusive
/* FP:init_mask.rs-0439 */             //
/* FP:init_mask.rs-0440 */             // For (a), the block index of `end_inclusive` is 1, and for (b), it's 0.
/* FP:init_mask.rs-0441 */             // This provides the desired behavior of searching blocks 0 and 1 for (a),
/* FP:init_mask.rs-0442 */             // and searching only block 0 for (b).
/* FP:init_mask.rs-0443 */             // There is no concern of overflows since we checked for `start >= end` above.
/* FP:init_mask.rs-0444 */             let (start_block, start_bit) = InitMaskMaterialized::bit_index(start);
/* FP:init_mask.rs-0445 */             let end_inclusive = Size::from_bytes(end.bytes() - 1);
/* FP:init_mask.rs-0446 */             let (end_block_inclusive, _) = InitMaskMaterialized::bit_index(end_inclusive);
/* FP:init_mask.rs-0447 */ 
/* FP:init_mask.rs-0448 */             // Handle first block: need to skip `start_bit` bits.
/* FP:init_mask.rs-0449 */             //
/* FP:init_mask.rs-0450 */             // We need to handle the first block separately,
/* FP:init_mask.rs-0451 */             // because there may be bits earlier in the block that should be ignored,
/* FP:init_mask.rs-0452 */             // such as the bit marked (1) in this example:
/* FP:init_mask.rs-0453 */             //
/* FP:init_mask.rs-0454 */             //       (1)
/* FP:init_mask.rs-0455 */             //       -|------
/* FP:init_mask.rs-0456 */             //   (c) 01000000|00000000|00000001
/* FP:init_mask.rs-0457 */             //          ^~~~~~~~~~~~~~~~~~^
/* FP:init_mask.rs-0458 */             //        start              end
/* FP:init_mask.rs-0459 */             if let Some(i) =
/* FP:init_mask.rs-0460 */                 search_block(init_mask.blocks[start_block], start_block, start_bit, is_init)
/* FP:init_mask.rs-0461 */             {
/* FP:init_mask.rs-0462 */                 // If the range is less than a block, we may find a matching bit after `end`.
/* FP:init_mask.rs-0463 */                 //
/* FP:init_mask.rs-0464 */                 // For example, we shouldn't successfully find bit (2), because it's after `end`:
/* FP:init_mask.rs-0465 */                 //
/* FP:init_mask.rs-0466 */                 //             (2)
/* FP:init_mask.rs-0467 */                 //       -------|
/* FP:init_mask.rs-0468 */                 //   (d) 00000001|00000000|00000001
/* FP:init_mask.rs-0469 */                 //        ^~~~~^
/* FP:init_mask.rs-0470 */                 //      start end
/* FP:init_mask.rs-0471 */                 //
/* FP:init_mask.rs-0472 */                 // An alternative would be to mask off end bits in the same way as we do for start bits,
/* FP:init_mask.rs-0473 */                 // but performing this check afterwards is faster and simpler to implement.
/* FP:init_mask.rs-0474 */                 if i < end {
/* FP:init_mask.rs-0475 */                     return Some(i);
/* FP:init_mask.rs-0476 */                 } else {
/* FP:init_mask.rs-0477 */                     return None;
/* FP:init_mask.rs-0478 */                 }
/* FP:init_mask.rs-0479 */             }
/* FP:init_mask.rs-0480 */ 
/* FP:init_mask.rs-0481 */             // Handle remaining blocks.
/* FP:init_mask.rs-0482 */             //
/* FP:init_mask.rs-0483 */             // We can skip over an entire block at once if it's all 0s (resp. 1s).
/* FP:init_mask.rs-0484 */             // The block marked (3) in this example is the first block that will be handled by this loop,
/* FP:init_mask.rs-0485 */             // and it will be skipped for that reason:
/* FP:init_mask.rs-0486 */             //
/* FP:init_mask.rs-0487 */             //                   (3)
/* FP:init_mask.rs-0488 */             //                --------
/* FP:init_mask.rs-0489 */             //   (e) 01000000|00000000|00000001
/* FP:init_mask.rs-0490 */             //          ^~~~~~~~~~~~~~~~~~^
/* FP:init_mask.rs-0491 */             //        start              end
/* FP:init_mask.rs-0492 */             if start_block < end_block_inclusive {
/* FP:init_mask.rs-0493 */                 // This loop is written in a specific way for performance.
/* FP:init_mask.rs-0494 */                 // Notably: `..end_block_inclusive + 1` is used for an inclusive range instead of `..=end_block_inclusive`,
/* FP:init_mask.rs-0495 */                 // and `.zip(start_block + 1..)` is used to track the index instead of `.enumerate().skip().take()`,
/* FP:init_mask.rs-0496 */                 // because both alternatives result in significantly worse codegen.
/* FP:init_mask.rs-0497 */                 // `end_block_inclusive + 1` is guaranteed not to wrap, because `end_block_inclusive <= end / BLOCK_SIZE`,
/* FP:init_mask.rs-0498 */                 // and `BLOCK_SIZE` (the number of bits per block) will always be at least 8 (1 byte).
/* FP:init_mask.rs-0499 */                 for (&bits, block) in init_mask.blocks[start_block + 1..end_block_inclusive + 1]
/* FP:init_mask.rs-0500 */                     .iter()
/* FP:init_mask.rs-0501 */                     .zip(start_block + 1..)
/* FP:init_mask.rs-0502 */                 {
/* FP:init_mask.rs-0503 */                     if let Some(i) = search_block(bits, block, 0, is_init) {
/* FP:init_mask.rs-0504 */                         // If this is the last block, we may find a matching bit after `end`.
/* FP:init_mask.rs-0505 */                         //
/* FP:init_mask.rs-0506 */                         // For example, we shouldn't successfully find bit (4), because it's after `end`:
/* FP:init_mask.rs-0507 */                         //
/* FP:init_mask.rs-0508 */                         //                               (4)
/* FP:init_mask.rs-0509 */                         //                         -------|
/* FP:init_mask.rs-0510 */                         //   (f) 00000001|00000000|00000001
/* FP:init_mask.rs-0511 */                         //          ^~~~~~~~~~~~~~~~~~^
/* FP:init_mask.rs-0512 */                         //        start              end
/* FP:init_mask.rs-0513 */                         //
/* FP:init_mask.rs-0514 */                         // As above with example (d), we could handle the end block separately and mask off end bits,
/* FP:init_mask.rs-0515 */                         // but unconditionally searching an entire block at once and performing this check afterwards
/* FP:init_mask.rs-0516 */                         // is faster and much simpler to implement.
/* FP:init_mask.rs-0517 */                         if i < end {
/* FP:init_mask.rs-0518 */                             return Some(i);
/* FP:init_mask.rs-0519 */                         } else {
/* FP:init_mask.rs-0520 */                             return None;
/* FP:init_mask.rs-0521 */                         }
/* FP:init_mask.rs-0522 */                     }
/* FP:init_mask.rs-0523 */                 }
/* FP:init_mask.rs-0524 */             }
/* FP:init_mask.rs-0525 */ 
/* FP:init_mask.rs-0526 */             None
/* FP:init_mask.rs-0527 */         }
/* FP:init_mask.rs-0528 */ 
/* FP:init_mask.rs-0529 */         #[cfg_attr(not(debug_assertions), allow(dead_code))]
/* FP:init_mask.rs-0530 */         fn find_bit_slow(
/* FP:init_mask.rs-0531 */             init_mask: &InitMaskMaterialized,
/* FP:init_mask.rs-0532 */             start: Size,
/* FP:init_mask.rs-0533 */             end: Size,
/* FP:init_mask.rs-0534 */             is_init: bool,
/* FP:init_mask.rs-0535 */         ) -> Option<Size> {
/* FP:init_mask.rs-0536 */             (start..end).find(|&i| init_mask.get(i) == is_init)
/* FP:init_mask.rs-0537 */         }
/* FP:init_mask.rs-0538 */ 
/* FP:init_mask.rs-0539 */         let result = find_bit_fast(self, start, end, is_init);
/* FP:init_mask.rs-0540 */ 
/* FP:init_mask.rs-0541 */         debug_assert_eq!(
/* FP:init_mask.rs-0542 */             result,
/* FP:init_mask.rs-0543 */             find_bit_slow(self, start, end, is_init),
/* FP:init_mask.rs-0544 */             "optimized implementation of find_bit is wrong for start={start:?} end={end:?} is_init={is_init} init_mask={self:#?}"
/* FP:init_mask.rs-0545 */         );
/* FP:init_mask.rs-0546 */ 
/* FP:init_mask.rs-0547 */         result
/* FP:init_mask.rs-0548 */     }
/* FP:init_mask.rs-0549 */ }
/* FP:init_mask.rs-0550 */ 
/* FP:init_mask.rs-0551 */ /// A contiguous chunk of initialized or uninitialized memory.
/* FP:init_mask.rs-0552 */ pub enum InitChunk {
/* FP:init_mask.rs-0553 */     Init(Range<Size>),
/* FP:init_mask.rs-0554 */     Uninit(Range<Size>),
/* FP:init_mask.rs-0555 */ }
/* FP:init_mask.rs-0556 */ 
/* FP:init_mask.rs-0557 */ impl InitChunk {
/* FP:init_mask.rs-0558 */     #[inline]
/* FP:init_mask.rs-0559 */     pub fn is_init(&self) -> bool {
/* FP:init_mask.rs-0560 */         match self {
/* FP:init_mask.rs-0561 */             Self::Init(_) => true,
/* FP:init_mask.rs-0562 */             Self::Uninit(_) => false,
/* FP:init_mask.rs-0563 */         }
/* FP:init_mask.rs-0564 */     }
/* FP:init_mask.rs-0565 */ 
/* FP:init_mask.rs-0566 */     #[inline]
/* FP:init_mask.rs-0567 */     pub fn range(&self) -> Range<Size> {
/* FP:init_mask.rs-0568 */         match self {
/* FP:init_mask.rs-0569 */             Self::Init(r) => r.clone(),
/* FP:init_mask.rs-0570 */             Self::Uninit(r) => r.clone(),
/* FP:init_mask.rs-0571 */         }
/* FP:init_mask.rs-0572 */     }
/* FP:init_mask.rs-0573 */ }
/* FP:init_mask.rs-0574 */ 
/* FP:init_mask.rs-0575 */ impl InitMask {
/* FP:init_mask.rs-0576 */     /// Returns an iterator, yielding a range of byte indexes for each contiguous region
/* FP:init_mask.rs-0577 */     /// of initialized or uninitialized bytes inside the range `start..end` (end-exclusive).
/* FP:init_mask.rs-0578 */     ///
/* FP:init_mask.rs-0579 */     /// The iterator guarantees the following:
/* FP:init_mask.rs-0580 */     /// - Chunks are nonempty.
/* FP:init_mask.rs-0581 */     /// - Chunks are adjacent (each range's start is equal to the previous range's end).
/* FP:init_mask.rs-0582 */     /// - Chunks span exactly `start..end` (the first starts at `start`, the last ends at `end`).
/* FP:init_mask.rs-0583 */     /// - Chunks alternate between [`InitChunk::Init`] and [`InitChunk::Uninit`].
/* FP:init_mask.rs-0584 */     #[inline]
/* FP:init_mask.rs-0585 */     pub fn range_as_init_chunks(&self, range: AllocRange) -> InitChunkIter<'_> {
/* FP:init_mask.rs-0586 */         let start = range.start;
/* FP:init_mask.rs-0587 */         let end = range.end();
/* FP:init_mask.rs-0588 */         assert!(end <= self.len);
/* FP:init_mask.rs-0589 */ 
/* FP:init_mask.rs-0590 */         let is_init = if start < end {
/* FP:init_mask.rs-0591 */             self.get(start)
/* FP:init_mask.rs-0592 */         } else {
/* FP:init_mask.rs-0593 */             // `start..end` is empty: there are no chunks, so use some arbitrary value
/* FP:init_mask.rs-0594 */             false
/* FP:init_mask.rs-0595 */         };
/* FP:init_mask.rs-0596 */ 
/* FP:init_mask.rs-0597 */         InitChunkIter { init_mask: self, is_init, start, end }
/* FP:init_mask.rs-0598 */     }
/* FP:init_mask.rs-0599 */ }
/* FP:init_mask.rs-0600 */ 
/* FP:init_mask.rs-0601 */ /// Yields [`InitChunk`]s. See [`InitMask::range_as_init_chunks`].
/* FP:init_mask.rs-0602 */ #[derive(Clone)]
/* FP:init_mask.rs-0603 */ pub struct InitChunkIter<'a> {
/* FP:init_mask.rs-0604 */     init_mask: &'a InitMask,
/* FP:init_mask.rs-0605 */     /// Whether the next chunk we will return is initialized.
/* FP:init_mask.rs-0606 */     /// If there are no more chunks, contains some arbitrary value.
/* FP:init_mask.rs-0607 */     is_init: bool,
/* FP:init_mask.rs-0608 */     /// The current byte index into `init_mask`.
/* FP:init_mask.rs-0609 */     start: Size,
/* FP:init_mask.rs-0610 */     /// The end byte index into `init_mask`.
/* FP:init_mask.rs-0611 */     end: Size,
/* FP:init_mask.rs-0612 */ }
/* FP:init_mask.rs-0613 */ 
/* FP:init_mask.rs-0614 */ impl<'a> Iterator for InitChunkIter<'a> {
/* FP:init_mask.rs-0615 */     type Item = InitChunk;
/* FP:init_mask.rs-0616 */ 
/* FP:init_mask.rs-0617 */     #[inline]
/* FP:init_mask.rs-0618 */     fn next(&mut self) -> Option<Self::Item> {
/* FP:init_mask.rs-0619 */         if self.start >= self.end {
/* FP:init_mask.rs-0620 */             return None;
/* FP:init_mask.rs-0621 */         }
/* FP:init_mask.rs-0622 */ 
/* FP:init_mask.rs-0623 */         let end_of_chunk = match self.init_mask.blocks {
/* FP:init_mask.rs-0624 */             InitMaskBlocks::Lazy { .. } => {
/* FP:init_mask.rs-0625 */                 // If we're iterating over the chunks of lazy blocks, we just emit a single
/* FP:init_mask.rs-0626 */                 // full-size chunk.
/* FP:init_mask.rs-0627 */                 self.end
/* FP:init_mask.rs-0628 */             }
/* FP:init_mask.rs-0629 */             InitMaskBlocks::Materialized(ref blocks) => {
/* FP:init_mask.rs-0630 */                 let end_of_chunk =
/* FP:init_mask.rs-0631 */                     blocks.find_bit(self.start, self.end, !self.is_init).unwrap_or(self.end);
/* FP:init_mask.rs-0632 */                 end_of_chunk
/* FP:init_mask.rs-0633 */             }
/* FP:init_mask.rs-0634 */         };
/* FP:init_mask.rs-0635 */         let range = self.start..end_of_chunk;
/* FP:init_mask.rs-0636 */         let ret =
/* FP:init_mask.rs-0637 */             Some(if self.is_init { InitChunk::Init(range) } else { InitChunk::Uninit(range) });
/* FP:init_mask.rs-0638 */ 
/* FP:init_mask.rs-0639 */         self.is_init = !self.is_init;
/* FP:init_mask.rs-0640 */         self.start = end_of_chunk;
/* FP:init_mask.rs-0641 */ 
/* FP:init_mask.rs-0642 */         ret
/* FP:init_mask.rs-0643 */     }
/* FP:init_mask.rs-0644 */ }
/* FP:init_mask.rs-0645 */ 
/* FP:init_mask.rs-0646 */ /// Run-length encoding of the uninit mask.
/* FP:init_mask.rs-0647 */ /// Used to copy parts of a mask multiple times to another allocation.
/* FP:init_mask.rs-0648 */ pub struct InitCopy {
/* FP:init_mask.rs-0649 */     /// Whether the first range is initialized.
/* FP:init_mask.rs-0650 */     initial: bool,
/* FP:init_mask.rs-0651 */     /// The lengths of ranges that are run-length encoded.
/* FP:init_mask.rs-0652 */     /// The initialization state of the ranges alternate starting with `initial`.
/* FP:init_mask.rs-0653 */     ranges: smallvec::SmallVec<[u64; 1]>,
/* FP:init_mask.rs-0654 */ }
/* FP:init_mask.rs-0655 */ 
/* FP:init_mask.rs-0656 */ impl InitCopy {
/* FP:init_mask.rs-0657 */     pub fn no_bytes_init(&self) -> bool {
/* FP:init_mask.rs-0658 */         // The `ranges` are run-length encoded and of alternating initialization state.
/* FP:init_mask.rs-0659 */         // So if `ranges.len() > 1` then the second block is an initialized range.
/* FP:init_mask.rs-0660 */         !self.initial && self.ranges.len() == 1
/* FP:init_mask.rs-0661 */     }
/* FP:init_mask.rs-0662 */ }
/* FP:init_mask.rs-0663 */ 
/* FP:init_mask.rs-0664 */ /// Transferring the initialization mask to other allocations.
/* FP:init_mask.rs-0665 */ impl InitMask {
/* FP:init_mask.rs-0666 */     /// Creates a run-length encoding of the initialization mask; panics if range is empty.
/* FP:init_mask.rs-0667 */     ///
/* FP:init_mask.rs-0668 */     /// This is essentially a more space-efficient version of
/* FP:init_mask.rs-0669 */     /// `InitMask::range_as_init_chunks(...).collect::<Vec<_>>()`.
/* FP:init_mask.rs-0670 */     pub fn prepare_copy(&self, range: AllocRange) -> InitCopy {
/* FP:init_mask.rs-0671 */         // Since we are copying `size` bytes from `src` to `dest + i * size` (`for i in 0..repeat`),
/* FP:init_mask.rs-0672 */         // a naive initialization mask copying algorithm would repeatedly have to read the initialization mask from
/* FP:init_mask.rs-0673 */         // the source and write it to the destination. Even if we optimized the memory accesses,
/* FP:init_mask.rs-0674 */         // we'd be doing all of this `repeat` times.
/* FP:init_mask.rs-0675 */         // Therefore we precompute a compressed version of the initialization mask of the source value and
/* FP:init_mask.rs-0676 */         // then write it back `repeat` times without computing any more information from the source.
/* FP:init_mask.rs-0677 */ 
/* FP:init_mask.rs-0678 */         // A precomputed cache for ranges of initialized / uninitialized bits
/* FP:init_mask.rs-0679 */         // 0000010010001110 will become
/* FP:init_mask.rs-0680 */         // `[5, 1, 2, 1, 3, 3, 1]`,
/* FP:init_mask.rs-0681 */         // where each element toggles the state.
/* FP:init_mask.rs-0682 */ 
/* FP:init_mask.rs-0683 */         let mut ranges = smallvec::SmallVec::<[u64; 1]>::new();
/* FP:init_mask.rs-0684 */ 
/* FP:init_mask.rs-0685 */         let mut chunks = self.range_as_init_chunks(range).peekable();
/* FP:init_mask.rs-0686 */ 
/* FP:init_mask.rs-0687 */         let initial = chunks.peek().expect("range should be nonempty").is_init();
/* FP:init_mask.rs-0688 */ 
/* FP:init_mask.rs-0689 */         // Here we rely on `range_as_init_chunks` to yield alternating init/uninit chunks.
/* FP:init_mask.rs-0690 */         for chunk in chunks {
/* FP:init_mask.rs-0691 */             let len = chunk.range().end.bytes() - chunk.range().start.bytes();
/* FP:init_mask.rs-0692 */             ranges.push(len);
/* FP:init_mask.rs-0693 */         }
/* FP:init_mask.rs-0694 */ 
/* FP:init_mask.rs-0695 */         InitCopy { ranges, initial }
/* FP:init_mask.rs-0696 */     }
/* FP:init_mask.rs-0697 */ 
/* FP:init_mask.rs-0698 */     /// Applies multiple instances of the run-length encoding to the initialization mask.
/* FP:init_mask.rs-0699 */     pub fn apply_copy(&mut self, defined: InitCopy, range: AllocRange, repeat: u64) {
/* FP:init_mask.rs-0700 */         // An optimization where we can just overwrite an entire range of initialization bits if
/* FP:init_mask.rs-0701 */         // they are going to be uniformly `1` or `0`. If this happens to be a full-range overwrite,
/* FP:init_mask.rs-0702 */         // we won't need materialized blocks either.
/* FP:init_mask.rs-0703 */         if defined.ranges.len() <= 1 {
/* FP:init_mask.rs-0704 */             let start = range.start;
/* FP:init_mask.rs-0705 */             let end = range.start + range.size * repeat; // `Size` operations
/* FP:init_mask.rs-0706 */             self.set_range(AllocRange::from(start..end), defined.initial);
/* FP:init_mask.rs-0707 */             return;
/* FP:init_mask.rs-0708 */         }
/* FP:init_mask.rs-0709 */ 
/* FP:init_mask.rs-0710 */         // We're about to do one or more partial writes, so we ensure the blocks are materialized.
/* FP:init_mask.rs-0711 */         let blocks = self.materialize_blocks();
/* FP:init_mask.rs-0712 */ 
/* FP:init_mask.rs-0713 */         for mut j in 0..repeat {
/* FP:init_mask.rs-0714 */             j *= range.size.bytes();
/* FP:init_mask.rs-0715 */             j += range.start.bytes();
/* FP:init_mask.rs-0716 */             let mut cur = defined.initial;
/* FP:init_mask.rs-0717 */             for range in &defined.ranges {
/* FP:init_mask.rs-0718 */                 let old_j = j;
/* FP:init_mask.rs-0719 */                 j += range;
/* FP:init_mask.rs-0720 */                 blocks.set_range_inbounds(Size::from_bytes(old_j), Size::from_bytes(j), cur);
/* FP:init_mask.rs-0721 */                 cur = !cur;
/* FP:init_mask.rs-0722 */             }
/* FP:init_mask.rs-0723 */         }
/* FP:init_mask.rs-0724 */     }
/* FP:init_mask.rs-0725 */ }