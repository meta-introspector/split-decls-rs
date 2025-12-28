macro_rules! deps {
    () => {
        Linker!();
    };
}

macro_rules! add_library_search_dirs {
    () => {
        deps!();
        # [doc = " Add sysroot and other globally set directories to the directory search list."] fn add_library_search_dirs (cmd : & mut dyn Linker , sess : & Session , self_contained_components : LinkSelfContainedComponents , apple_sdk_root : Option < & Path > ,) { if ! sess . opts . unstable_opts . link_native_libraries { return ; } let fallback = Some (NativeLibSearchFallback { self_contained_components , apple_sdk_root }) ; let _ = walk_native_lib_search_dirs (sess , fallback , | dir , is_framework | { if is_framework { cmd . framework_path (dir) ; } else { cmd . include_path (& fix_windows_verbatim_for_gcc (dir)) ; } ControlFlow :: < () > :: Continue (()) }) ; }
    };
}

add_library_search_dirs!()