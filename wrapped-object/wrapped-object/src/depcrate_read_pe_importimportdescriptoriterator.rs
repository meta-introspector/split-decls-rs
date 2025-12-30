// Generated macro for ImportDescriptorIterator (struct)
macro_rules! Depcrate_read_pe_importImportDescriptorIterator {
() => {
// Module: crate::read::pe::import
// Provides: {"ImportDescriptorIterator"}
// Dependencies: {}
# [doc = " A fallible iterator for the descriptors in the import data directory."] # [derive (Debug , Clone)] pub struct ImportDescriptorIterator < 'data > { data : Bytes < 'data > , null : bool , }
};
}
