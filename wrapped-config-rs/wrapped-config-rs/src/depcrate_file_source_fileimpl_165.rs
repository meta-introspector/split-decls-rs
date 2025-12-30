// Generated macro for impl_165 (impl)
macro_rules! Depcrate_file_source_fileimpl_165 {
() => {
// Module: crate::file::source::file
// Provides: {"impl_165"}
// Dependencies: {}
impl < F > FileSource < F > for FileSourceFile where F : Format + FileStoredFormat + 'static , { fn resolve (& self , format_hint : Option < F > ,) -> Result < FileSourceResult , Box < dyn Error + Send + Sync > > { let (filename , format) = self . find_file (format_hint) ? ; let uri = env :: current_dir () . ok () . and_then (| base | pathdiff :: diff_paths (& filename , base)) . unwrap_or_else (| | filename . clone ()) ; let buf = fs :: read (filename) ? ; let buf = if buf . len () >= 3 && & buf [0 .. 3] == b"\xef\xbb\xbf" { & buf [3 ..] } else { & buf } ; let c = String :: from_utf8_lossy (buf) ; let text = c . into_owned () ; Ok (FileSourceResult { uri : Some (uri . to_string_lossy () . into_owned ()) , content : text , format , }) } }
};
}
