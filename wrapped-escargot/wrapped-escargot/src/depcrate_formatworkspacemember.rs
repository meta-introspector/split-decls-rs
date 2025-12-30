// Generated macro for WorkspaceMember (struct)
macro_rules! Depcrate_formatWorkspaceMember {
() => {
// Module: crate::format
// Provides: {"WorkspaceMember"}
// Dependencies: {}
# [doc = " A workspace member. This is basically identical to `cargo::core::package_id::PackageId`, except"] # [doc = " that this does not use `Arc` internally."] # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Serialize , Deserialize)] # [serde (transparent)] # [cfg_attr (feature = "strict_unstable" , serde (deny_unknown_fields))] pub struct WorkspaceMember < 'a > { # [doc = " The raw package id as given by cargo"] # [serde (borrow)] raw : CowStr < 'a > , }
};
}
