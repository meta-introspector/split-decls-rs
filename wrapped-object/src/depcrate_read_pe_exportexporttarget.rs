// Generated macro for ExportTarget (enum)
macro_rules! Depcrate_read_pe_exportExportTarget {
() => {
// Module: crate::read::pe::export
// Provides: {"ExportTarget"}
// Dependencies: {}
# [doc = " Where an export is pointing to."] # [derive (Clone , Copy)] pub enum ExportTarget < 'data > { # [doc = " The address of the export, relative to the image base."] Address (u32) , # [doc = " Forwarded to an export ordinal in another DLL."] # [doc = ""] # [doc = " This gives the name of the DLL, and the ordinal."] ForwardByOrdinal (& 'data [u8] , u32) , # [doc = " Forwarded to an export name in another DLL."] # [doc = ""] # [doc = " This gives the name of the DLL, and the export name."] ForwardByName (& 'data [u8] , & 'data [u8]) , }
};
}
