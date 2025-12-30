// Generated macro for Export (struct)
macro_rules! Depcrate_read_pe_exportExport {
() => {
// Module: crate::read::pe::export
// Provides: {"Export"}
// Dependencies: {}
# [doc = " An export from a PE file."] # [doc = ""] # [doc = " There are multiple kinds of PE exports (with or without a name, and local or forwarded)."] # [derive (Clone , Copy)] pub struct Export < 'data > { # [doc = " The ordinal of the export."] # [doc = ""] # [doc = " These are sequential, starting at a base specified in the DLL."] pub ordinal : u32 , # [doc = " The name of the export, if known."] pub name : Option < & 'data [u8] > , # [doc = " The target of this export."] pub target : ExportTarget < 'data > , }
};
}
