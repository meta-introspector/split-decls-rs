// Generated macro for impl_1049 (impl)
macro_rules! Depcrate_read_pe_resourceimpl_1049 {
() => {
// Module: crate::read::pe::resource
// Provides: {"impl_1049"}
// Dependencies: {}
impl ResourceName { # [doc = " Converts to a `String`."] pub fn to_string_lossy (& self , directory : ResourceDirectory < '_ >) -> Result < String > { let d = self . data (directory) ? . iter () . map (| c | c . get (LE)) ; Ok (char :: decode_utf16 (d) . map (| r | r . unwrap_or (char :: REPLACEMENT_CHARACTER)) . collect :: < String > ()) } # [doc = " Returns the string unicode buffer."] pub fn data < 'data > (& self , directory : ResourceDirectory < 'data > ,) -> Result < & 'data [U16Bytes < LE >] > { let mut offset = u64 :: from (self . offset) ; let len = directory . data . read :: < U16Bytes < LE > > (& mut offset) . read_error ("Invalid resource name offset") ? ; directory . data . read_slice :: < U16Bytes < LE > > (& mut offset , len . get (LE) . into ()) . read_error ("Invalid resource name length") } # [doc = " Returns the string buffer as raw bytes."] pub fn raw_data < 'data > (& self , directory : ResourceDirectory < 'data >) -> Result < & 'data [u8] > { self . data (directory) . map (crate :: pod :: bytes_of_slice) } }
};
}
