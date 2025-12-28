macro_rules! get_rust_lib_src {
    () => {
        fn get_rust_lib_src (sysroot_path : & AbsPath) -> Option < AbsPathBuf > { let rust_lib_src = sysroot_path . join ("lib/rustlib/src/rust/library") ; tracing :: debug ! ("checking sysroot library: {rust_lib_src}") ; if fs :: metadata (& rust_lib_src) . is_ok () { Some (rust_lib_src) } else { None } }
    };
}

get_rust_lib_src!();