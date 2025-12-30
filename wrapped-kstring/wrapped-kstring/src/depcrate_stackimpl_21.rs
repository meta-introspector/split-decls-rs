// Generated macro for impl_21 (impl)
macro_rules! Depcrate_stackimpl_21 {
() => {
// Module: crate::stack
// Provides: {"impl_21"}
// Dependencies: {}
impl < const CAPACITY : usize > fmt :: Display for StackString < CAPACITY > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self . as_str () , f) } }
};
}
