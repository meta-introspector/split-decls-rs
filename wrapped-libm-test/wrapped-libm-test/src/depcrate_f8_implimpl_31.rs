// Generated macro for impl_31 (impl)
macro_rules! Depcrate_f8_implimpl_31 {
() => {
// Module: crate::f8_impl
// Provides: {"impl_31"}
// Dependencies: {}
impl cmp :: PartialEq for f8 { fn eq (& self , other : & Self) -> bool { if self . is_nan () || other . is_nan () { false } else if self . abs () . to_bits () | other . abs () . to_bits () == 0 { true } else { self . 0 == other . 0 } } }
};
}
