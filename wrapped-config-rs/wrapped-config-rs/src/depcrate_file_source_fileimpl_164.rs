// Generated macro for impl_164 (impl)
macro_rules! Depcrate_file_source_fileimpl_164 {
() => {
// Module: crate::file::source::file
// Provides: {"impl_164"}
// Dependencies: {}
impl FileSourceFile { pub fn new (name : PathBuf) -> Self { Self { name } } fn find_file < F > (& self , format_hint : Option < F > ,) -> Result < (PathBuf , Box < dyn Format >) , Box < dyn Error + Send + Sync > > where F : FileStoredFormat + Format + 'static , { let path = if self . name . is_absolute () { self . name . clone () } else { env :: current_dir () ? . as_path () . join (& self . name) } ; if path . is_file () { if let Some (format) = format_hint { return Ok ((path , Box :: new (format))) ; } else { let ext = path . extension () . unwrap_or_default () . to_string_lossy () ; for format in FileFormat :: all () { if format . extensions () . contains (& ext . as_ref ()) { return Ok ((path , Box :: new (* format))) ; } } return Err (Box :: new (io :: Error :: new (io :: ErrorKind :: NotFound , format ! ("configuration file \"{}\" is not of a supported file format" , path . to_string_lossy ()) ,))) ; } ; } let mut path = path ; if path . extension () . is_some () { path . as_mut_os_string () . push (".placeholder") ; } match format_hint { Some (format) => { for ext in format . file_extensions () { path . set_extension (ext) ; if path . is_file () { return Ok ((path , Box :: new (format))) ; } } } None => { for format in FileFormat :: all () { for ext in format . extensions () { path . set_extension (ext) ; if path . is_file () { return Ok ((path , Box :: new (* format))) ; } } } } } Err (Box :: new (io :: Error :: new (io :: ErrorKind :: NotFound , format ! ("configuration file \"{}\" not found" , self . name . to_string_lossy ()) ,))) } }
};
}
