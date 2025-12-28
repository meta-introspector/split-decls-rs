macro_rules! deps {
    () => {
        IMAGE_SECTION_CHARACTERISTICS!();
    };
}

macro_rules! IMAGE_SECTION_HEADER {
    () => {
        deps!();
        # [repr (C)] # [derive (Clone , Copy)] pub struct IMAGE_SECTION_HEADER { pub Name : [u8 ; 8] , pub Misc : IMAGE_SECTION_HEADER_0 , pub VirtualAddress : u32 , pub SizeOfRawData : u32 , pub PointerToRawData : u32 , pub PointerToRelocations : u32 , pub PointerToLinenumbers : u32 , pub NumberOfRelocations : u16 , pub NumberOfLinenumbers : u16 , pub Characteristics : IMAGE_SECTION_CHARACTERISTICS , }
    };
}

IMAGE_SECTION_HEADER!()