// Generated macro for impl_12 (impl)
macro_rules! Depcrate_stackimpl_12 {
() => {
// Module: crate::stack
// Provides: {"impl_12"}
// Dependencies: {}
impl < const CAPACITY : usize > PartialEq < & str > for StackString < CAPACITY > { # [inline] fn eq (& self , other : & & str) -> bool { PartialEq :: eq (self . as_str () , * other) } }
};
}
