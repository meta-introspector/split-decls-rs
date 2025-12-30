// Generated macro for impl_171 (impl)
macro_rules! Depcrate_file_source_stringimpl_171 {
() => {
// Module: crate::file::source::string
// Provides: {"impl_171"}
// Dependencies: {}
impl < F > FileSource < F > for FileSourceString where F : Format + FileStoredFormat + 'static , { fn resolve (& self , format_hint : Option < F > ,) -> Result < FileSourceResult , Box < dyn Error + Send + Sync > > { Ok (FileSourceResult { uri : None , content : self . 0 . clone () , format : Box :: new (format_hint . expect ("from_str requires a set file format")) , }) } }
};
}
