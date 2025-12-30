// Generated macro for BuildScript (struct)
macro_rules! Depcrate_formatBuildScript {
() => {
// Module: crate::format
// Provides: {"BuildScript"}
// Dependencies: {}
# [doc = " Output of a Build Script execution."] # [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] # [cfg_attr (feature = "strict_unstable" , serde (deny_unknown_fields))] # [non_exhaustive] pub struct BuildScript < 'a > { # [doc = " The workspace member this build script execution belongs to"] # [serde (borrow)] pub package_id : WorkspaceMember < 'a > , # [doc = " The outdir used."] # [serde (borrow)] # [serde (default)] pub out_dir : Option < CowPath < 'a > > , # [doc = " The libs to link"] # [serde (borrow)] pub linked_libs : Vec < CowStr < 'a > > , # [doc = " The paths to search when resolving libs"] # [serde (borrow)] pub linked_paths : Vec < CowPath < 'a > > , # [doc = " The paths to search when resolving libs"] # [serde (borrow)] pub cfgs : Vec < CowPath < 'a > > , # [doc = " The environment variables to add to the compilation"] # [serde (borrow)] pub env : Vec < (CowStr < 'a > , CowStr < 'a >) > , }
};
}
