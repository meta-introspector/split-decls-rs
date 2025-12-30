// Generated macro for Options (struct)
macro_rules! Depcrate_export_fs_exporterOptions {
() => {
// Module: crate::export::fs_exporter
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Options bag for initializing a [`FilesystemExporter`]."] # [non_exhaustive] # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] pub struct Options { # [doc = " Directory in the filesystem to write output."] pub root : PathBuf , # [doc = " Option for initializing the output directory."] pub overwrite : OverwriteOption , }
};
}
