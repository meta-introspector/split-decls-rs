// Generated macro for impl_20 (impl)
macro_rules! Depcrate_stackimpl_20 {
() => {
// Module: crate::stack
// Provides: {"impl_20"}
// Dependencies: {}
impl < const CAPACITY : usize > fmt :: Debug for StackString < CAPACITY > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . as_str () , f) } }
};
}
