// Generated macro for CoffFile (struct)
macro_rules! Depcrate_read_coff_fileCoffFile {
() => {
// Module: crate::read::coff::file
// Provides: {"CoffFile"}
// Dependencies: {}
# [doc = " A COFF object file."] # [doc = ""] # [doc = " This is a file that starts with [`pe::ImageFileHeader`], and corresponds"] # [doc = " to [`crate::FileKind::Coff`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`Object`] trait implementation."] # [derive (Debug)] pub struct CoffFile < 'data , R : ReadRef < 'data > = & 'data [u8] , Coff : CoffHeader = pe :: ImageFileHeader > { pub (super) header : & 'data Coff , pub (super) common : CoffCommon < 'data , R , Coff > , pub (super) data : R , }
};
}
