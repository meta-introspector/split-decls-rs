// Generated macro for DelayLoadImportTable (struct)
macro_rules! Depcrate_read_pe_importDelayLoadImportTable {
() => {
// Module: crate::read::pe::import
// Provides: {"DelayLoadImportTable"}
// Dependencies: {}
# [doc = " Information for parsing a PE delay-load import table."] # [doc = ""] # [doc = " Returned by"] # [doc = " [`DataDirectories::delay_load_import_table`](super::DataDirectories::delay_load_import_table)."] # [derive (Debug , Clone)] pub struct DelayLoadImportTable < 'data > { section_data : Bytes < 'data > , section_address : u32 , import_address : u32 , }
};
}
