// Generated macro for impl_733 (impl)
macro_rules! Depcrate_open_permissionsimpl_733 {
() => {
// Module: crate::open::permissions
// Provides: {"impl_733"}
// Dependencies: {}
impl Attributes { # [doc = " Allow everything which usually relates to a fully trusted environment"] pub fn all () -> Self { Attributes { git_binary : false , system : true , git : true , } } # [doc = " Allow loading attributes that are local to the git repository."] pub fn isolated () -> Self { Attributes { git_binary : false , system : false , git : false , } } }
};
}
