// Generated macro for Import (enum)
macro_rules! Depcrate_read_pe_importImport {
() => {
// Module: crate::read::pe::import
// Provides: {"Import"}
// Dependencies: {}
# [doc = " A parsed import thunk."] # [derive (Debug , Clone , Copy)] pub enum Import < 'data > { # [doc = " Import by ordinal."] Ordinal (u16) , # [doc = " Import by name."] # [doc = ""] # [doc = " Includes a hint for the index into the export name pointer table in the target library."] Name (u16 , & 'data [u8]) , }
};
}
