// Generated macro for impl_45 (impl)
macro_rules! Depcrate_export_fs_exporterimpl_45 {
() => {
// Module: crate::export::fs_exporter
// Provides: {"impl_45"}
// Dependencies: {}
impl FilesystemExporter { # [doc = " Creates a new [`FilesystemExporter`] with a [serializer] and [options]."] # [doc = ""] # [doc = " See the module-level docs for an example."] # [doc = ""] # [doc = " [serializer]: crate::export::serializers"] # [doc = " [options]: Options"] pub fn try_new (serializer : Box < dyn AbstractSerializer + Sync > , options : Options ,) -> Result < Self , DataError > { let result = FilesystemExporter { root : options . root , manifest : Manifest :: for_format (serializer . get_buffer_format ()) ? , serializer , } ; match options . overwrite { OverwriteOption :: CheckEmpty if result . root . exists () => fs :: remove_dir (& result . root) , OverwriteOption :: RemoveAndReplace if result . root . exists () => { fs :: remove_dir_all (& result . root) } _ => Ok (()) , } . and_then (| _ | fs :: create_dir_all (& result . root)) . map_err (| e | DataError :: from (e) . with_path_context (& result . root)) ? ; result . manifest . write (& result . root) ? ; Ok (result) } fn setup_file (& self , mut path_buf : PathBuf) -> Result < Box < dyn std :: io :: Write > , DataError > { path_buf . set_extension (self . manifest . file_extension) ; let file : Box < dyn std :: io :: Write > = if self . serializer . is_text_format () { Box :: new (crlify :: BufWriterWithLineEndingFix :: new (fs :: File :: create (& path_buf) . map_err (| e | DataError :: from (e) . with_path_context (& path_buf)) ? ,)) } else { Box :: new (std :: io :: BufWriter :: new (fs :: File :: create (& path_buf) . map_err (| e | DataError :: from (e) . with_path_context (& path_buf)) ? ,)) } ; Ok (file) } }
};
}
