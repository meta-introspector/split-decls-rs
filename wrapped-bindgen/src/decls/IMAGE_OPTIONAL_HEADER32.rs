macro_rules! deps {
    () => {
        IMAGE_DATA_DIRECTORY!();
        IMAGE_SUBSYSTEM!();
        IMAGE_OPTIONAL_HEADER_MAGIC!();
        IMAGE_DLL_CHARACTERISTICS!();
    };
}

macro_rules! IMAGE_OPTIONAL_HEADER32 {
    () => {
        deps!();
        # [repr (C)] # [derive (Clone , Copy)] pub struct IMAGE_OPTIONAL_HEADER32 { pub Magic : IMAGE_OPTIONAL_HEADER_MAGIC , pub MajorLinkerVersion : u8 , pub MinorLinkerVersion : u8 , pub SizeOfCode : u32 , pub SizeOfInitializedData : u32 , pub SizeOfUninitializedData : u32 , pub AddressOfEntryPoint : u32 , pub BaseOfCode : u32 , pub BaseOfData : u32 , pub ImageBase : u32 , pub SectionAlignment : u32 , pub FileAlignment : u32 , pub MajorOperatingSystemVersion : u16 , pub MinorOperatingSystemVersion : u16 , pub MajorImageVersion : u16 , pub MinorImageVersion : u16 , pub MajorSubsystemVersion : u16 , pub MinorSubsystemVersion : u16 , pub Win32VersionValue : u32 , pub SizeOfImage : u32 , pub SizeOfHeaders : u32 , pub CheckSum : u32 , pub Subsystem : IMAGE_SUBSYSTEM , pub DllCharacteristics : IMAGE_DLL_CHARACTERISTICS , pub SizeOfStackReserve : u32 , pub SizeOfStackCommit : u32 , pub SizeOfHeapReserve : u32 , pub SizeOfHeapCommit : u32 , pub LoaderFlags : u32 , pub NumberOfRvaAndSizes : u32 , pub DataDirectory : [IMAGE_DATA_DIRECTORY ; 16] , }
    };
}

IMAGE_OPTIONAL_HEADER32!();