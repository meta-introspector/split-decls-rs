// Generated macro for impl_74 (impl)
macro_rules! Depcrateimpl_74 {
() => {
// Module: crate
// Provides: {"impl_74"}
// Dependencies: {}
impl < A , B > Default for AbsDiff < A , B > where A : AbsDiffEq < B > + ? Sized , B : ? Sized , { # [inline] fn default () -> AbsDiff < A , B > { AbsDiff { epsilon : A :: default_epsilon () , } } }
};
}
