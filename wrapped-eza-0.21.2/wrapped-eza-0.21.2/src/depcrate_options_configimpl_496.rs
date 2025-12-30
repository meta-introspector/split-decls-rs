// Generated macro for impl_496 (impl)
macro_rules! Depcrate_options_configimpl_496 {
() => {
// Module: crate::options::config
// Provides: {"impl_496"}
// Dependencies: {}
impl FromOverride < SELinuxContextOverride > for SELinuxContext { fn from (value : SELinuxContextOverride , default : Self) -> Self { SELinuxContext { colon : FromOverride :: from (value . colon , default . colon) , user : FromOverride :: from (value . user , default . user) , role : FromOverride :: from (value . role , default . role) , typ : FromOverride :: from (value . typ , default . typ) , range : FromOverride :: from (value . range , default . range) , } } }
};
}
