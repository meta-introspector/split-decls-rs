macro_rules! deps {
    () => {
        ManifestPath!();
    };
}

macro_rules! get_rustc_src {
    () => {
        deps!();
        fn get_rustc_src (sysroot_path : & AbsPath) -> Option < ManifestPath > { let rustc_src = sysroot_path . join ("lib/rustlib/rustc-src/rust/compiler/rustc/Cargo.toml") ; let rustc_src = ManifestPath :: try_from (rustc_src) . ok () ? ; tracing :: debug ! ("checking for rustc source code: {rustc_src}") ; if fs :: metadata (& rustc_src) . is_ok () { Some (rustc_src) } else { None } }
    };
}

get_rustc_src!();