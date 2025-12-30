// Generated macro for impl_488 (impl)
macro_rules! Depcrate_options_configimpl_488 {
() => {
// Module: crate::options::config
// Provides: {"impl_488"}
// Dependencies: {}
impl FromOverride < UsersOverride > for Users { fn from (value : UsersOverride , default : Self) -> Self { Users { user_you : FromOverride :: from (value . user_you , default . user_you) , user_root : FromOverride :: from (value . user_root , default . user_root) , user_other : FromOverride :: from (value . user_other , default . user_other) , group_yours : FromOverride :: from (value . group_yours , default . group_yours) , group_other : FromOverride :: from (value . group_other , default . group_other) , group_root : FromOverride :: from (value . group_root , default . group_root) , } } }
};
}
