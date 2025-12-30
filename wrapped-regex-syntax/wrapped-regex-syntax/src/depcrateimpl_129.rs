// Generated macro for impl_129 (impl)
macro_rules! Depcrateimpl_129 {
() => {
// Module: crate
// Provides: {"impl_129"}
// Dependencies: {}
impl PartialOrd < ByteRange > for u8 { # [inline] fn partial_cmp (& self , other : & ByteRange) -> Option < Ordering > { other . partial_cmp (self) . map (| o | o . reverse ()) } }
};
}
