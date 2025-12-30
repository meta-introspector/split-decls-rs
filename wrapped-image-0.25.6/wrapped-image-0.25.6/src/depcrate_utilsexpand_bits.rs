// Generated macro for expand_bits (function)
macro_rules! Depcrate_utilsexpand_bits {
() => {
// Module: crate::utils
// Provides: {"expand_bits"}
// Dependencies: {}
# [doc = " Expand a buffer of packed 1, 2, or 4 bits integers into u8's. Assumes that"] # [doc = " every `row_size` entries there are padding bits up to the next byte boundary."] # [allow (dead_code)] pub (crate) fn expand_bits (bit_depth : u8 , row_size : u32 , buf : & [u8]) -> Vec < u8 > { let mask = (1u8 << bit_depth as usize) - 1 ; let scaling_factor = 255 / ((1 << bit_depth as usize) - 1) ; let bit_width = row_size * u32 :: from (bit_depth) ; let skip = if bit_width % 8 == 0 { 0 } else { (8 - bit_width % 8) / u32 :: from (bit_depth) } ; let row_len = row_size + skip ; let mut p = Vec :: new () ; let mut i = 0 ; for v in buf { for shift_inv in 1 ..= 8 / bit_depth { let shift = 8 - bit_depth * shift_inv ; if i % (row_len as usize) < (row_size as usize) { let pixel = (v & (mask << shift as usize)) >> shift as usize ; p . push (pixel * scaling_factor) ; } i += 1 ; } } p }
};
}
