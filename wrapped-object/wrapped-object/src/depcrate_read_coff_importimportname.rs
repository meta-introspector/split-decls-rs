// Generated macro for ImportName (enum)
macro_rules! Depcrate_read_coff_importImportName {
() => {
// Module: crate::read::coff::import
// Provides: {"ImportName"}
// Dependencies: {}
# [doc = " The name or ordinal to import from a DLL."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum ImportName < 'data > { # [doc = " Import by ordinal. Ordinarily this is a 1-based index."] Ordinal (u16) , # [doc = " Import by name."] Name (& 'data [u8]) , }
};
}
