macro_rules! discover_rust_lib_src_dir {
    () => {
        fn discover_rust_lib_src_dir (sysroot_path : & AbsPathBuf) -> Option < AbsPathBuf > { if let Ok (path) = env :: var ("RUST_SRC_PATH") { if let Ok (path) = AbsPathBuf :: try_from (path . as_str ()) { let core = path . join ("core") ; if fs :: metadata (& core) . is_ok () { tracing :: debug ! ("Discovered sysroot by RUST_SRC_PATH: {path}") ; return Some (path) ; } tracing :: debug ! ("RUST_SRC_PATH is set, but is invalid (no core: {core:?}), ignoring") ; } else { tracing :: debug ! ("RUST_SRC_PATH is set, but is invalid, ignoring") ; } } get_rust_lib_src (sysroot_path) }
    };
}

discover_rust_lib_src_dir!()