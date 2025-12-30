// Generated macro for impl_46 (impl)
macro_rules! Depcrate_traitsimpl_46 {
() => {
// Module: crate::traits
// Provides: {"impl_46"}
// Dependencies: {}
impl U16UtfExt for u16 { # [inline] fn utf16_needs_extra_unit (self) -> Result < bool , Utf16FirstUnitError > { match self { 0x00_00 ..= 0xd7_ff | 0xe0_00 ..= 0xff_ff => Ok (false) , 0xd8_00 ..= 0xdb_ff => Ok (true) , _ => Err (Utf16FirstUnitError) } } # [inline] fn is_utf16_leading_surrogate (self) -> bool { (self & 0xfc00) == 0xd800 } }
};
}
