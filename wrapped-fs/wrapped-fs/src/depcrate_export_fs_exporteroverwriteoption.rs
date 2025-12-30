// Generated macro for OverwriteOption (enum)
macro_rules! Depcrate_export_fs_exporterOverwriteOption {
() => {
// Module: crate::export::fs_exporter
// Provides: {"OverwriteOption"}
// Dependencies: {}
# [doc = " Choices of what to do if [`FilesystemExporter`] tries to write to a pre-existing directory."] # [non_exhaustive] # [derive (Copy , Clone , Debug , PartialEq , Serialize , Deserialize)] pub enum OverwriteOption { # [doc = " If the directory doesn't exist, create it."] # [doc = " If it does exist, remove it safely (`rmdir`) and re-create it."] CheckEmpty , # [doc = " If the directory doesn't exist, create it."] # [doc = " If it does exist, remove it aggressively (`rm -rf`) and re-create it."] RemoveAndReplace , }
};
}
