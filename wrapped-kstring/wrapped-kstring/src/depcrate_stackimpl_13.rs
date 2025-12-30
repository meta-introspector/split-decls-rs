// Generated macro for impl_13 (impl)
macro_rules! Depcrate_stackimpl_13 {
() => {
// Module: crate::stack
// Provides: {"impl_13"}
// Dependencies: {}
impl < const CAPACITY : usize > PartialEq < String > for StackString < CAPACITY > { # [inline] fn eq (& self , other : & String) -> bool { PartialEq :: eq (self . as_str () , other . as_str ()) } }
};
}
