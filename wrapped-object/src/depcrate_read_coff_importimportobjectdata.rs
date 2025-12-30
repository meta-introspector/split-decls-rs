// Generated macro for ImportObjectData (struct)
macro_rules! Depcrate_read_coff_importImportObjectData {
() => {
// Module: crate::read::coff::import
// Provides: {"ImportObjectData"}
// Dependencies: {}
# [doc = " The data following [`pe::ImportObjectHeader`]."] # [derive (Debug , Clone)] pub struct ImportObjectData < 'data > { symbol : ByteString < 'data > , dll : ByteString < 'data > , export : Option < ByteString < 'data > > , }
};
}
