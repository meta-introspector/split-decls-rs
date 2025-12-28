macro_rules! deps {
    () => {
        PCWSTR!();
        BOOL!();
        SearchPath!();
    };
}

macro_rules! enum_loaded_modules_callback {
    () => {
        deps!();
        extern "system" fn enum_loaded_modules_callback (module_name : PCWSTR , _ : u64 , _ : u32 , user_context : * const c_void ,) -> BOOL { let len : usize = unsafe { lstrlenW (module_name) . try_into () . unwrap () } ; if len == 0 { return TRUE ; } let module_name = unsafe { slice :: from_raw_parts (module_name , len) } ; let path_sep = utf16_char ('\\') ; let alt_path_sep = utf16_char ('/') ; let Some (end_of_directory) = module_name . iter () . rposition (| & c | c == path_sep || c == alt_path_sep) else { return TRUE ; } ; let search_path = unsafe { & mut * (user_context as * mut SearchPath) } ; search_path . add (& module_name [.. end_of_directory]) ; TRUE }
    };
}

enum_loaded_modules_callback!();