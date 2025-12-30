// Generated macro for impl_16 (impl)
macro_rules! Depcrate_bfloatimpl_16 {
() => {
// Module: crate::bfloat
// Provides: {"impl_16"}
// Dependencies: {}
impl PartialEq for bf16 { fn eq (& self , other : & bf16) -> bool { if self . is_nan () || other . is_nan () { false } else { (self . 0 == other . 0) || ((self . 0 | other . 0) & 0x7FFFu16 == 0) } } }
};
}
