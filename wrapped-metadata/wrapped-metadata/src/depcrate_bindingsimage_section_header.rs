// Generated macro for IMAGE_SECTION_HEADER (struct)
macro_rules! Depcrate_bindingsIMAGE_SECTION_HEADER {
() => {
// Module: crate::bindings
// Provides: {"IMAGE_SECTION_HEADER"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy)] pub struct IMAGE_SECTION_HEADER { pub Name : [u8 ; 8] , pub Misc : IMAGE_SECTION_HEADER_0 , pub VirtualAddress : u32 , pub SizeOfRawData : u32 , pub PointerToRawData : u32 , pub PointerToRelocations : u32 , pub PointerToLinenumbers : u32 , pub NumberOfRelocations : u16 , pub NumberOfLinenumbers : u16 , pub Characteristics : IMAGE_SECTION_CHARACTERISTICS , }
};
}
