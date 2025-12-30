// Generated macro for impl_142 (impl)
macro_rules! Depcrate_astimpl_142 {
() => {
// Module: crate::ast
// Provides: {"impl_142"}
// Dependencies: {}
impl < N , M : Default > Type < N , M > { # [doc = " Creates a new `null`able [`Type`] literal from the provided `name`."] # [must_use] pub fn nullable (name : impl Into < N >) -> Self { Self { name : name . into () , modifiers : M :: default () , } } }
};
}
