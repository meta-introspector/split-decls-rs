// Generated macro for impl_81 (impl)
macro_rules! Depcrate_binary16impl_81 {
() => {
// Module: crate::binary16
// Provides: {"impl_81"}
// Dependencies: {}
impl PartialEq for f16 { fn eq (& self , other : & f16) -> bool { if self . is_nan () || other . is_nan () { false } else { (self . 0 == other . 0) || ((self . 0 | other . 0) & 0x7FFFu16 == 0) } } }
};
}
