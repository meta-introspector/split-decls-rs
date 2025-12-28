macro_rules! deps {
    () => {
        Linker!();
    };
}

macro_rules! link_sanitizer_runtime {
    () => {
        deps!();
        fn link_sanitizer_runtime (sess : & Session , flavor : LinkerFlavor , linker : & mut dyn Linker , name : & str ,) { fn find_sanitizer_runtime (sess : & Session , filename : & str) -> PathBuf { let path = sess . target_tlib_path . dir . join (filename) ; if path . exists () { sess . target_tlib_path . dir . clone () } else { filesearch :: make_target_lib_path (& sess . opts . sysroot . default , sess . opts . target_triple . tuple () ,) } } let channel = option_env ! ("CFG_RELEASE_CHANNEL") . map (| channel | format ! ("-{channel}")) . unwrap_or_default () ; if sess . target . is_like_darwin { let filename = format ! ("rustc{channel}_rt.{name}") ; let path = find_sanitizer_runtime (sess , & filename) ; let rpath = path . to_str () . expect ("non-utf8 component in path") ; linker . link_args (& ["-rpath" , rpath]) ; linker . link_dylib_by_name (& filename , false , true) ; } else if sess . target . is_like_msvc && flavor == LinkerFlavor :: Msvc (Lld :: No) && name == "asan" { linker . link_arg ("/INFERASANLIBS") ; } else { let filename = format ! ("librustc{channel}_rt.{name}.a") ; let path = find_sanitizer_runtime (sess , & filename) . join (& filename) ; linker . link_staticlib_by_path (& path , true) ; } }
    };
}

link_sanitizer_runtime!()