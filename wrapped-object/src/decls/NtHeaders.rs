macro_rules! NtHeaders {
    () => {
        # [doc = " Information required for writing [`pe::ImageNtHeaders32`] or [`pe::ImageNtHeaders64`]."] # [allow (missing_docs)] # [derive (Debug , Clone)] pub struct NtHeaders { pub machine : u16 , pub time_date_stamp : u32 , pub characteristics : u16 , pub major_linker_version : u8 , pub minor_linker_version : u8 , pub address_of_entry_point : u32 , pub image_base : u64 , pub major_operating_system_version : u16 , pub minor_operating_system_version : u16 , pub major_image_version : u16 , pub minor_image_version : u16 , pub major_subsystem_version : u16 , pub minor_subsystem_version : u16 , pub subsystem : u16 , pub dll_characteristics : u16 , pub size_of_stack_reserve : u64 , pub size_of_stack_commit : u64 , pub size_of_heap_reserve : u64 , pub size_of_heap_commit : u64 , }
    };
}

NtHeaders!()