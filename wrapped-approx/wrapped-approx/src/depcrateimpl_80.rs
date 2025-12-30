// Generated macro for impl_80 (impl)
macro_rules! Depcrateimpl_80 {
() => {
// Module: crate
// Provides: {"impl_80"}
// Dependencies: {}
impl < A , B > Default for Ulps < A , B > where A : UlpsEq < B > + ? Sized , B : ? Sized , { # [inline] fn default () -> Ulps < A , B > { Ulps { epsilon : A :: default_epsilon () , max_ulps : A :: default_max_ulps () , } } }
};
}
