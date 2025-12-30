// Generated macro for impl_16 (impl)
macro_rules! Depcrate_stackimpl_16 {
() => {
// Module: crate::stack
// Provides: {"impl_16"}
// Dependencies: {}
impl < const CAPACITY : usize > PartialOrd < str > for StackString < CAPACITY > { # [inline] fn partial_cmp (& self , other : & str) -> Option < std :: cmp :: Ordering > { self . as_str () . partial_cmp (other) } }
};
}
