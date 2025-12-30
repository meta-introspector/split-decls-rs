// Generated macro for impl_58 (impl)
macro_rules! Depcrate_literalimpl_58 {
() => {
// Module: crate::literal
// Provides: {"impl_58"}
// Dependencies: {}
impl From < u8 > for CChar { fn from (i : u8) -> CChar { match i { 0 ..= 0x7f => CChar :: Char (i as u8 as char) , _ => CChar :: Raw (i as u64) , } } }
};
}
