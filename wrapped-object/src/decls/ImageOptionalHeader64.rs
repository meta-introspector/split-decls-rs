macro_rules! deps {
    () => {
        U32!();
        U16!();
        U64!();
    };
}

macro_rules! ImageOptionalHeader64 {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageOptionalHeader64 { pub magic : U16 < LE > , pub major_linker_version : u8 , pub minor_linker_version : u8 , pub size_of_code : U32 < LE > , pub size_of_initialized_data : U32 < LE > , pub size_of_uninitialized_data : U32 < LE > , pub address_of_entry_point : U32 < LE > , pub base_of_code : U32 < LE > , pub image_base : U64 < LE > , pub section_alignment : U32 < LE > , pub file_alignment : U32 < LE > , pub major_operating_system_version : U16 < LE > , pub minor_operating_system_version : U16 < LE > , pub major_image_version : U16 < LE > , pub minor_image_version : U16 < LE > , pub major_subsystem_version : U16 < LE > , pub minor_subsystem_version : U16 < LE > , pub win32_version_value : U32 < LE > , pub size_of_image : U32 < LE > , pub size_of_headers : U32 < LE > , pub check_sum : U32 < LE > , pub subsystem : U16 < LE > , pub dll_characteristics : U16 < LE > , pub size_of_stack_reserve : U64 < LE > , pub size_of_stack_commit : U64 < LE > , pub size_of_heap_reserve : U64 < LE > , pub size_of_heap_commit : U64 < LE > , pub loader_flags : U32 < LE > , pub number_of_rva_and_sizes : U32 < LE > , }
    };
}

ImageOptionalHeader64!()