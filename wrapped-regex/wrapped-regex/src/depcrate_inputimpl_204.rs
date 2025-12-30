// Generated macro for impl_204 (impl)
macro_rules! Depcrate_inputimpl_204 {
() => {
// Module: crate::input
// Provides: {"impl_204"}
// Dependencies: {}
impl From < Option < char > > for Char { fn from (c : Option < char >) -> Char { c . map_or (Char (u32 :: MAX) , | c | c . into ()) } }
};
}
