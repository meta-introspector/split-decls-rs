// Generated macro for RangeReader (trait)
macro_rules! Depcrate_range_decRangeReader {
() => {
// Module: crate::range_dec
// Provides: {"RangeReader"}
// Dependencies: {}
pub (crate) trait RangeReader { fn read_u8 (& mut self) -> u8 ; fn try_read_u8 (& mut self) -> crate :: Result < u8 > ; fn read_u32_be (& mut self) -> crate :: Result < u32 > ; # [inline (always)] fn is_buffer (& self) -> bool { false } # [inline (always)] fn pos (& self) -> usize { unimplemented ! ("not a buffer reader") } # [inline (always)] fn set_pos (& mut self , _pos : usize) { unimplemented ! ("not a buffer reader") } # [inline (always)] fn buf (& self) -> & [u8] { unimplemented ! ("not a buffer reader") } }
};
}
