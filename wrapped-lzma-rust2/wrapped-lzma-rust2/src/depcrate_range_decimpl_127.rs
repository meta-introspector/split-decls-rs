// Generated macro for impl_127 (impl)
macro_rules! Depcrate_range_decimpl_127 {
() => {
// Module: crate::range_dec
// Provides: {"impl_127"}
// Dependencies: {}
impl RangeDecoder < RangeDecoderBuffer > { pub (crate) fn prepare < R : Read + ByteReader > (& mut self , mut reader : R , len : usize ,) -> crate :: Result < () > { if len < 5 { return Err (error_invalid_input ("buffer len must >= 5")) ; } let b = reader . read_u8 () ? ; if b != 0x00 { return Err (error_invalid_input ("first byte is 0")) ; } self . code = reader . read_u32_be () ? ; self . range = 0xFFFFFFFFu32 ; let len = len - 5 ; let pos = self . inner . buf . len () - len ; let end = pos + len ; self . inner . pos = pos ; reader . read_exact (& mut self . inner . buf [pos .. end]) } # [inline] pub (crate) fn is_finished (& self) -> bool { self . inner . pos == self . inner . buf . len () && self . code == 0 } }
};
}
