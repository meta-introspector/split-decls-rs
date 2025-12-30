// Generated macro for impl_197 (impl)
macro_rules! Depcrate_fileimpl_197 {
() => {
// Module: crate::file
// Provides: {"impl_197"}
// Dependencies: {}
impl < T , F > Source for File < T , F > where F : FileStoredFormat + Debug + Clone + Send + Sync + 'static , T : Sync + Send + FileSource < F > + 'static , { fn clone_into_box (& self) -> Box < dyn Source + Send + Sync > { Box :: new ((* self) . clone ()) } fn collect (& self) -> Result < Map < String , Value > > { let (uri , contents , format) = match self . source . resolve (self . format . clone ()) . map_err (ConfigError :: Foreign) { Ok (result) => (result . uri , result . content , result . format) , Err (error) => { if ! self . required { return Ok (Map :: new ()) ; } return Err (error) ; } } ; format . parse (uri . as_ref () , & contents) . map_err (| cause | ConfigError :: FileParse { uri , cause }) } }
};
}
