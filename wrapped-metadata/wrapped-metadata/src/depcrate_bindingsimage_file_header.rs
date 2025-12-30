// Generated macro for IMAGE_FILE_HEADER (struct)
macro_rules! Depcrate_bindingsIMAGE_FILE_HEADER {
() => {
// Module: crate::bindings
// Provides: {"IMAGE_FILE_HEADER"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Default)] pub struct IMAGE_FILE_HEADER { pub Machine : IMAGE_FILE_MACHINE , pub NumberOfSections : u16 , pub TimeDateStamp : u32 , pub PointerToSymbolTable : u32 , pub NumberOfSymbols : u32 , pub SizeOfOptionalHeader : u16 , pub Characteristics : IMAGE_FILE_CHARACTERISTICS , }
};
}
