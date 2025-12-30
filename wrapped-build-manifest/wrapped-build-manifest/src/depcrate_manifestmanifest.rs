// Generated macro for Manifest (struct)
macro_rules! Depcrate_manifestManifest {
() => {
// Module: crate::manifest
// Provides: {"Manifest"}
// Dependencies: {}
# [derive (Serialize)] # [serde (rename_all = "kebab-case")] pub (crate) struct Manifest { pub (crate) manifest_version : String , pub (crate) date : String , pub (crate) pkg : BTreeMap < String , Package > , pub (crate) artifacts : BTreeMap < String , Artifact > , pub (crate) renames : BTreeMap < String , Rename > , pub (crate) profiles : BTreeMap < String , Vec < String > > , }
};
}
