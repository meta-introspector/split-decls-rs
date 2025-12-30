// Generated macro for impl_274 (impl)
macro_rules! Depcrate_oddimpl_274 {
() => {
// Module: crate::odd
// Provides: {"impl_274"}
// Dependencies: {}
impl < T > Odd < T > { # [doc = " Create a new odd integer."] pub fn new (n : T) -> CtOption < Self > where T : Integer , { let is_odd = n . is_odd () ; CtOption :: new (Self (n) , is_odd) } # [doc = " Returns the inner value."] pub fn get (self) -> T { self . 0 } }
};
}
