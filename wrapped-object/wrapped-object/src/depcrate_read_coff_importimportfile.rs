// Generated macro for ImportFile (struct)
macro_rules! Depcrate_read_coff_importImportFile {
() => {
// Module: crate::read::coff::import
// Provides: {"ImportFile"}
// Dependencies: {}
# [doc = " A Windows short form description of a symbol to import."] # [doc = ""] # [doc = " Used in Windows import libraries to provide a mapping from"] # [doc = " a symbol name to a DLL export. This is not an object file."] # [doc = ""] # [doc = " This is a file that starts with [`pe::ImportObjectHeader`], and corresponds"] # [doc = " to [`crate::FileKind::CoffImport`]."] # [derive (Debug , Clone)] pub struct ImportFile < 'data > { header : & 'data pe :: ImportObjectHeader , kind : ImportType , dll : ByteString < 'data > , symbol : ByteString < 'data > , import : Option < ByteString < 'data > > , }
};
}
