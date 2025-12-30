// Generated macro for impl_77 (impl)
macro_rules! Depcrateimpl_77 {
() => {
// Module: crate
// Provides: {"impl_77"}
// Dependencies: {}
impl < A , B > Default for Relative < A , B > where A : RelativeEq < B > + ? Sized , B : ? Sized , { # [inline] fn default () -> Relative < A , B > { Relative { epsilon : A :: default_epsilon () , max_relative : A :: default_max_relative () , } } }
};
}
