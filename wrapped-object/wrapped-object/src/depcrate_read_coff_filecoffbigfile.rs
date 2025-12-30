// Generated macro for CoffBigFile (type)
macro_rules! Depcrate_read_coff_fileCoffBigFile {
() => {
// Module: crate::read::coff::file
// Provides: {"CoffBigFile"}
// Dependencies: {}
# [doc = " A COFF bigobj object file with 32-bit section numbers."] # [doc = ""] # [doc = " This is a file that starts with [`pe::AnonObjectHeaderBigobj`], and corresponds"] # [doc = " to [`crate::FileKind::CoffBig`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`Object`] trait implementation."] pub type CoffBigFile < 'data , R = & 'data [u8] > = CoffFile < 'data , R , pe :: AnonObjectHeaderBigobj > ;
};
}
