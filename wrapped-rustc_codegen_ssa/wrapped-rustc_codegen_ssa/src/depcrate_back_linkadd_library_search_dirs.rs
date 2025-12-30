// Generated macro for add_library_search_dirs (function)
macro_rules! Depcrate_back_linkadd_library_search_dirs {
() => {
// Module: crate::back::link
// Provides: {"add_library_search_dirs"}
// Dependencies: {}
# [doc = " Add sysroot and other globally set directories to the directory search list."] fn add_library_search_dirs (cmd : & mut dyn Linker , sess : & Session , self_contained_components : LinkSelfContainedComponents , apple_sdk_root : Option < & Path > ,) { if ! sess . opts . unstable_opts . link_native_libraries { return ; } let fallback = Some (NativeLibSearchFallback { self_contained_components , apple_sdk_root }) ; let _ = walk_native_lib_search_dirs (sess , fallback , | dir , is_framework | { if is_framework { cmd . framework_path (dir) ; } else { cmd . include_path (& fix_windows_verbatim_for_gcc (dir)) ; } ControlFlow :: < () > :: Continue (()) }) ; }
};
}
