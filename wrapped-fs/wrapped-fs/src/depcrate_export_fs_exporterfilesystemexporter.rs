// Generated macro for FilesystemExporter (struct)
macro_rules! Depcrate_export_fs_exporterFilesystemExporter {
() => {
// Module: crate::export::fs_exporter
// Provides: {"FilesystemExporter"}
// Dependencies: {}
# [doc = " A data exporter that writes data to a filesystem hierarchy."] # [doc = " See the module-level docs for an example."] # [derive (Debug)] pub struct FilesystemExporter { root : PathBuf , manifest : Manifest , serializer : Box < dyn AbstractSerializer + Sync > , }
};
}
