// Generated macro for impl_221 (impl)
macro_rules! Depcrate_non_zeroimpl_221 {
() => {
// Module: crate::non_zero
// Provides: {"impl_221"}
// Dependencies: {}
impl < T > NonZero < T > { # [doc = " Create a new non-zero integer."] pub fn new (n : T) -> CtOption < Self > where T : Zero , { let is_zero = n . is_zero () ; CtOption :: new (Self (n) , ! is_zero) } # [doc = " Returns the inner value."] pub fn get (self) -> T { self . 0 } }
};
}
