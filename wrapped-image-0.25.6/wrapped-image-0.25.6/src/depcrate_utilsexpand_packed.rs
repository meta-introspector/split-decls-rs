// Generated macro for expand_packed (function)
macro_rules! Depcrate_utilsexpand_packed {
() => {
// Module: crate::utils
// Provides: {"expand_packed"}
// Dependencies: {}
# [inline (always)] pub (crate) fn expand_packed < F > (buf : & mut [u8] , channels : usize , bit_depth : u8 , mut func : F) where F : FnMut (u8 , & mut [u8]) , { let pixels = buf . len () / channels * bit_depth as usize ; let extra = pixels % 8 ; let entries = pixels / 8 + match extra { 0 => 0 , _ => 1 , } ; let mask = ((1u16 << bit_depth) - 1) as u8 ; let i = (0 .. entries) . rev () . flat_map (| idx | (0 .. 8 / bit_depth) . map (| i | i * bit_depth) . zip (repeat (idx))) . skip (extra) ; let buf_len = buf . len () ; let j_inv = (channels .. buf_len) . step_by (channels) ; for ((shift , i) , j_inv) in i . zip (j_inv) { let j = buf_len - j_inv ; let pixel = (buf [i] & (mask << shift)) >> shift ; func (pixel , & mut buf [j .. (j + channels)]) ; } }
};
}
