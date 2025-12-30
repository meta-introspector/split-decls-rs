// Generated macro for impl_205 (impl)
macro_rules! Depcrate_common_access_control_allow_methodsimpl_205 {
() => {
// Module: crate::common::access_control_allow_methods
// Provides: {"impl_205"}
// Dependencies: {}
impl AccessControlAllowMethods { # [doc = " Returns an iterator over `Method`s contained within."] pub fn iter (& self) -> impl Iterator < Item = Method > + '_ { self . 0 . iter () . filter_map (| s | s . parse () . ok ()) } }
};
}
