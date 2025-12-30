// Generated macro for impl_194 (impl)
macro_rules! Depcrate_fileimpl_194 {
() => {
// Module: crate::file
// Provides: {"impl_194"}
// Dependencies: {}
impl < T , F > File < T , F > where F : FileStoredFormat + 'static , T : FileSource < F > , { pub fn format (mut self , format : F) -> Self { self . format = Some (format) ; self } # [doc = " Set required to false to make a file optional when building the config."] pub fn required (mut self , required : bool) -> Self { self . required = required ; self } }
};
}
