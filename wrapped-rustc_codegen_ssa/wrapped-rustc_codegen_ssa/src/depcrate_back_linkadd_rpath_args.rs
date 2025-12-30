// Generated macro for add_rpath_args (function)
macro_rules! Depcrate_back_linkadd_rpath_args {
() => {
// Module: crate::back::link
// Provides: {"add_rpath_args"}
// Dependencies: {}
# [doc = " Add library search paths used at runtime by dynamic linkers."] fn add_rpath_args (cmd : & mut dyn Linker , sess : & Session , codegen_results : & CodegenResults , out_filename : & Path ,) { if ! sess . target . has_rpath { return ; } if sess . opts . cg . rpath { let libs = codegen_results . crate_info . used_crates . iter () . filter_map (| cnum | { codegen_results . crate_info . used_crate_source [cnum] . dylib . as_ref () . map (| (path , _) | & * * path) }) . collect :: < Vec < _ > > () ; let rpath_config = RPathConfig { libs : & * libs , out_filename : out_filename . to_path_buf () , is_like_darwin : sess . target . is_like_darwin , linker_is_gnu : sess . target . linker_flavor . is_gnu () , } ; cmd . link_args (& rpath :: get_rpath_linker_args (& rpath_config)) ; } }
};
}
