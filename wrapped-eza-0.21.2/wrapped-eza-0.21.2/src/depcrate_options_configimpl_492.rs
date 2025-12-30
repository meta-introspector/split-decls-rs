// Generated macro for impl_492 (impl)
macro_rules! Depcrate_options_configimpl_492 {
() => {
// Module: crate::options::config
// Provides: {"impl_492"}
// Dependencies: {}
impl FromOverride < GitOverride > for Git { fn from (value : GitOverride , default : Self) -> Self { Git { new : FromOverride :: from (value . new , default . new) , modified : FromOverride :: from (value . modified , default . modified) , deleted : FromOverride :: from (value . deleted , default . deleted) , renamed : FromOverride :: from (value . renamed , default . renamed) , typechange : FromOverride :: from (value . typechange , default . typechange) , ignored : FromOverride :: from (value . ignored , default . ignored) , conflicted : FromOverride :: from (value . conflicted , default . conflicted) , } } }
};
}
