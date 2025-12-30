// Generated macro for impl_17 (impl)
macro_rules! Depcrate_permissionimpl_17 {
() => {
// Module: crate::permission
// Provides: {"impl_17"}
// Dependencies: {}
impl Permission { # [doc = " Return true if this instance is `Permission::Allow`."] pub fn is_allowed (& self) -> bool { matches ! (self , Permission :: Allow) } # [doc = " Check this permissions and produce a reply to indicate if the `resource` can be used and in which way."] # [doc = ""] # [doc = " Only if this permission is set to `Allow` will the resource be usable."] pub fn check < R : std :: fmt :: Debug > (& self , resource : R) -> Result < Option < R > , Error < R > > { match self { Permission :: Allow => Ok (Some (resource)) , Permission :: Deny => Ok (None) , Permission :: Forbid => Err (Error { resource }) , } } # [doc = " Like [`check()`][Self::check()], but degenerates the type to an option to make it more useful in cases where"] # [doc = " `Forbid` shouldn't abort the entire operation."] pub fn check_opt < R : std :: fmt :: Debug > (& self , resource : R) -> Option < R > { match self { Permission :: Allow => Some (resource) , Permission :: Deny | Permission :: Forbid => None , } } }
};
}
