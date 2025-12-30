// Generated macro for impl_494 (impl)
macro_rules! Depcrate_options_configimpl_494 {
() => {
// Module: crate::options::config
// Provides: {"impl_494"}
// Dependencies: {}
impl FromOverride < GitRepoOverride > for GitRepo { fn from (value : GitRepoOverride , default : Self) -> Self { GitRepo { branch_main : FromOverride :: from (value . branch_main , default . branch_main) , branch_other : FromOverride :: from (value . branch_other , default . branch_other) , git_clean : FromOverride :: from (value . git_clean , default . git_clean) , git_dirty : FromOverride :: from (value . git_dirty , default . git_dirty) , } } }
};
}
