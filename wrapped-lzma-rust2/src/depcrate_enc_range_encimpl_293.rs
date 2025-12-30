// Generated macro for impl_293 (impl)
macro_rules! Depcrate_enc_range_encimpl_293 {
() => {
// Module: crate::enc::range_enc
// Provides: {"impl_293"}
// Dependencies: {}
impl RangeEncoder < () > { # [inline (always)] pub (crate) fn get_bit_price (prob : u32 , bit : i32) -> u32 { debug_assert ! (bit == 0 || bit == 1) ; let i = (prob ^ ((- bit) as u32 & (BIT_MODEL_TOTAL - 1))) >> MOVE_REDUCING_BITS ; PRICES [i as usize] as u32 } pub (crate) fn get_bit_tree_price (probs : & mut [u16] , symbol : u32) -> u32 { let mut price = 0 ; let mut symbol = symbol | probs . len () as u32 ; loop { let bit = symbol & 1 ; symbol >>= 1 ; price += Self :: get_bit_price (probs [symbol as usize] as u32 , bit as i32) ; if symbol == 1 { break ; } } price } pub (crate) fn get_reverse_bit_tree_price (probs : & mut [u16] , symbol : u32) -> u32 { let mut price = 0 ; let mut index = 1u32 ; let mut symbol = symbol | probs . len () as u32 ; loop { let bit = symbol & 1 ; symbol >>= 1 ; price += Self :: get_bit_price (probs [index as usize] as u32 , bit as i32) ; index = (index << 1) | bit ; if symbol == 1 { break ; } } price } # [inline] pub (crate) fn get_direct_bits_price (count : u32) -> u32 { count << BIT_PRICE_SHIFT_BITS } }
};
}
