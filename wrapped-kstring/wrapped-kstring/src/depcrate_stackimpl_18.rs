// Generated macro for impl_18 (impl)
macro_rules! Depcrate_stackimpl_18 {
() => {
// Module: crate::stack
// Provides: {"impl_18"}
// Dependencies: {}
impl < const CAPACITY : usize > PartialOrd < String > for StackString < CAPACITY > { # [inline] fn partial_cmp (& self , other : & String) -> Option < std :: cmp :: Ordering > { self . as_str () . partial_cmp (other . as_str ()) } }
};
}
