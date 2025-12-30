// Generated macro for Package (struct)
macro_rules! Depcrate_manifestPackage {
() => {
// Module: crate::manifest
// Provides: {"Package"}
// Dependencies: {}
# [derive (Serialize)] pub (crate) struct Package { pub (crate) version : String , pub (crate) git_commit_hash : Option < String > , pub (crate) target : BTreeMap < String , Target > , }
};
}
