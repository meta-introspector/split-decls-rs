// Generated macro for impl_498 (impl)
macro_rules! Depcrate_options_configimpl_498 {
() => {
// Module: crate::options::config
// Provides: {"impl_498"}
// Dependencies: {}
impl FromOverride < SecurityContextOverride > for SecurityContext { fn from (value : SecurityContextOverride , default : Self) -> Self { SecurityContext { none : FromOverride :: from (value . none , default . none) , selinux : FromOverride :: from (value . selinux , default . selinux) , } } }
};
}
