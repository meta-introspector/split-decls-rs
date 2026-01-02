
macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    fn main () { # [cfg (windows)] { # [link (name = "kernel32" , kind = "raw-dylib")] extern "C" { fn GetModuleFileNameA (module : * mut std :: ffi :: c_void , filename : * mut u8 , size : u32 ,) -> u32 ; } let mut buffer = [0u8 ; 1024] ; let size = unsafe { GetModuleFileNameA (core :: ptr :: null_mut () , buffer . as_mut_ptr () , buffer . len () as u32) } ; if size == 0 { eprintln ! ("failed to get module file name: {}" , std :: io :: Error :: last_os_error ()) ; return ; } else { let filename = std :: ffi :: CStr :: from_bytes_with_nul (& buffer [.. size as usize + 1]) . unwrap () ; assert ! (filename . to_str () . unwrap () . ends_with ("raw-dylib.exe")) ; } } }
}