// Generated macro for impl_730 (impl)
macro_rules! Depcrate_open_permissionsimpl_730 {
() => {
// Module: crate::open::permissions
// Provides: {"impl_730"}
// Dependencies: {}
impl Config { # [doc = " Allow everything which usually relates to a fully trusted environment"] pub fn all () -> Self { Config { git_binary : false , system : true , git : true , user : true , env : true , includes : true , } } # [doc = " Load only configuration local to the git repository."] pub fn isolated () -> Self { Config { git_binary : false , system : false , git : false , user : false , env : false , includes : false , } } }
};
}
