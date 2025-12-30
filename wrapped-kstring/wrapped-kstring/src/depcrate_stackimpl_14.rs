// Generated macro for impl_14 (impl)
macro_rules! Depcrate_stackimpl_14 {
() => {
// Module: crate::stack
// Provides: {"impl_14"}
// Dependencies: {}
impl < const CAPACITY : usize > Ord for StackString < CAPACITY > { # [inline] fn cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . as_str () . cmp (other . as_str ()) } }
};
}
