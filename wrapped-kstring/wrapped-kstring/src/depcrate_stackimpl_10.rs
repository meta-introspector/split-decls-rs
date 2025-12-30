// Generated macro for impl_10 (impl)
macro_rules! Depcrate_stackimpl_10 {
() => {
// Module: crate::stack
// Provides: {"impl_10"}
// Dependencies: {}
impl < const C1 : usize , const C2 : usize > PartialEq < StackString < C1 > > for StackString < C2 > { # [inline] fn eq (& self , other : & StackString < C1 >) -> bool { PartialEq :: eq (self . as_str () , other . as_str ()) } }
};
}
