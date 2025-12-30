// Generated macro for ExportTable (struct)
macro_rules! Depcrate_read_pe_exportExportTable {
() => {
// Module: crate::read::pe::export
// Provides: {"ExportTable"}
// Dependencies: {}
# [doc = " A partially parsed PE export table."] # [doc = ""] # [doc = " Returned by [`DataDirectories::export_table`](super::DataDirectories::export_table)."] # [derive (Debug , Clone)] pub struct ExportTable < 'data > { data : Bytes < 'data > , virtual_address : u32 , directory : & 'data pe :: ImageExportDirectory , addresses : & 'data [U32Bytes < LE >] , names : & 'data [U32Bytes < LE >] , name_ordinals : & 'data [U16Bytes < LE >] , }
};
}
