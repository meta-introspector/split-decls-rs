// Generated macro for impl_116 (impl)
macro_rules! Depcrateimpl_116 {
() => {
// Module: crate
// Provides: {"impl_116"}
// Dependencies: {}
impl < T : ConstOne + ConstZero > Complex < T > { # [doc = " A constant `Complex` 1."] pub const ONE : Self = Self :: new (T :: ONE , T :: ZERO) ; # [doc = " A constant `Complex` _i_, the imaginary unit."] pub const I : Self = Self :: new (T :: ZERO , T :: ONE) ; }
};
}
