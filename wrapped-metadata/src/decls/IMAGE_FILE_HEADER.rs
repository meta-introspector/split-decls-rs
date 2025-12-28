macro_rules! deps {
    () => {
        IMAGE_FILE_CHARACTERISTICS!();
        IMAGE_FILE_MACHINE!();
    };
}

macro_rules! IMAGE_FILE_HEADER {
    () => {
        deps!();
        # [repr (C)] # [derive (Clone , Copy , Default)] pub struct IMAGE_FILE_HEADER { pub Machine : IMAGE_FILE_MACHINE , pub NumberOfSections : u16 , pub TimeDateStamp : u32 , pub PointerToSymbolTable : u32 , pub NumberOfSymbols : u32 , pub SizeOfOptionalHeader : u16 , pub Characteristics : IMAGE_FILE_CHARACTERISTICS , }
    };
}

IMAGE_FILE_HEADER!()