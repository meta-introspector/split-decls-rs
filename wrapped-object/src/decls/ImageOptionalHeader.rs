macro_rules! deps {
    () => {
        Pod!();
    };
}

macro_rules! ImageOptionalHeader {
    () => {
        deps!();
        # [doc = " A trait for generic access to [`pe::ImageOptionalHeader32`] and [`pe::ImageOptionalHeader64`]."] # [allow (missing_docs)] pub trait ImageOptionalHeader : Debug + Pod { fn magic (& self) -> u16 ; fn major_linker_version (& self) -> u8 ; fn minor_linker_version (& self) -> u8 ; fn size_of_code (& self) -> u32 ; fn size_of_initialized_data (& self) -> u32 ; fn size_of_uninitialized_data (& self) -> u32 ; fn address_of_entry_point (& self) -> u32 ; fn base_of_code (& self) -> u32 ; fn base_of_data (& self) -> Option < u32 > ; fn image_base (& self) -> u64 ; fn section_alignment (& self) -> u32 ; fn file_alignment (& self) -> u32 ; fn major_operating_system_version (& self) -> u16 ; fn minor_operating_system_version (& self) -> u16 ; fn major_image_version (& self) -> u16 ; fn minor_image_version (& self) -> u16 ; fn major_subsystem_version (& self) -> u16 ; fn minor_subsystem_version (& self) -> u16 ; fn win32_version_value (& self) -> u32 ; fn size_of_image (& self) -> u32 ; fn size_of_headers (& self) -> u32 ; fn check_sum (& self) -> u32 ; fn subsystem (& self) -> u16 ; fn dll_characteristics (& self) -> u16 ; fn size_of_stack_reserve (& self) -> u64 ; fn size_of_stack_commit (& self) -> u64 ; fn size_of_heap_reserve (& self) -> u64 ; fn size_of_heap_commit (& self) -> u64 ; fn loader_flags (& self) -> u32 ; fn number_of_rva_and_sizes (& self) -> u32 ; }
    };
}

ImageOptionalHeader!();