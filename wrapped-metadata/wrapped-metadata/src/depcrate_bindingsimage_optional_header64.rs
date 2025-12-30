// Generated macro for IMAGE_OPTIONAL_HEADER64 (struct)
macro_rules! Depcrate_bindingsIMAGE_OPTIONAL_HEADER64 {
() => {
// Module: crate::bindings
// Provides: {"IMAGE_OPTIONAL_HEADER64"}
// Dependencies: {}
# [repr (C , packed (4))] # [derive (Clone , Copy)] pub struct IMAGE_OPTIONAL_HEADER64 { pub Magic : IMAGE_OPTIONAL_HEADER_MAGIC , pub MajorLinkerVersion : u8 , pub MinorLinkerVersion : u8 , pub SizeOfCode : u32 , pub SizeOfInitializedData : u32 , pub SizeOfUninitializedData : u32 , pub AddressOfEntryPoint : u32 , pub BaseOfCode : u32 , pub ImageBase : u64 , pub SectionAlignment : u32 , pub FileAlignment : u32 , pub MajorOperatingSystemVersion : u16 , pub MinorOperatingSystemVersion : u16 , pub MajorImageVersion : u16 , pub MinorImageVersion : u16 , pub MajorSubsystemVersion : u16 , pub MinorSubsystemVersion : u16 , pub Win32VersionValue : u32 , pub SizeOfImage : u32 , pub SizeOfHeaders : u32 , pub CheckSum : u32 , pub Subsystem : IMAGE_SUBSYSTEM , pub DllCharacteristics : IMAGE_DLL_CHARACTERISTICS , pub SizeOfStackReserve : u64 , pub SizeOfStackCommit : u64 , pub SizeOfHeapReserve : u64 , pub SizeOfHeapCommit : u64 , pub LoaderFlags : u32 , pub NumberOfRvaAndSizes : u32 , pub DataDirectory : [IMAGE_DATA_DIRECTORY ; 16] , }
};
}
