// Generated macro for ImportThunkList (struct)
macro_rules! Depcrate_read_pe_importImportThunkList {
() => {
// Module: crate::read::pe::import
// Provides: {"ImportThunkList"}
// Dependencies: {}
# [doc = " A list of import thunks."] # [doc = ""] # [doc = " These may be in the import lookup table, or the import address table."] # [derive (Debug , Clone)] pub struct ImportThunkList < 'data > { data : Bytes < 'data > , }
};
}
