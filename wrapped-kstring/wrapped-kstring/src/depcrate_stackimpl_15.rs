// Generated macro for impl_15 (impl)
macro_rules! Depcrate_stackimpl_15 {
() => {
// Module: crate::stack
// Provides: {"impl_15"}
// Dependencies: {}
impl < const C1 : usize , const C2 : usize > PartialOrd < StackString < C1 > > for StackString < C2 > { # [inline] fn partial_cmp (& self , other : & StackString < C1 >) -> Option < std :: cmp :: Ordering > { self . as_str () . partial_cmp (other . as_str ()) } }
};
}
