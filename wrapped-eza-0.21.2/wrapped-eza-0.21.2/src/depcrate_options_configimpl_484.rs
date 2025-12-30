// Generated macro for impl_484 (impl)
macro_rules! Depcrate_options_configimpl_484 {
() => {
// Module: crate::options::config
// Provides: {"impl_484"}
// Dependencies: {}
impl FromOverride < PermissionsOverride > for Permissions { fn from (value : PermissionsOverride , default : Self) -> Self { Permissions { user_read : FromOverride :: from (value . user_read , default . user_read) , user_write : FromOverride :: from (value . user_write , default . user_write) , user_execute_file : FromOverride :: from (value . user_execute_file , default . user_execute_file ,) , user_execute_other : FromOverride :: from (value . user_execute_other , default . user_execute_other ,) , group_read : FromOverride :: from (value . group_read , default . group_read) , group_write : FromOverride :: from (value . group_write , default . group_write) , group_execute : FromOverride :: from (value . group_execute , default . group_execute) , other_read : FromOverride :: from (value . other_read , default . other_read) , other_write : FromOverride :: from (value . other_write , default . other_write) , other_execute : FromOverride :: from (value . other_execute , default . other_execute) , special_user_file : FromOverride :: from (value . special_user_file , default . special_user_file ,) , special_other : FromOverride :: from (value . special_other , default . special_other) , attribute : FromOverride :: from (value . attribute , default . attribute) , } } }
};
}
