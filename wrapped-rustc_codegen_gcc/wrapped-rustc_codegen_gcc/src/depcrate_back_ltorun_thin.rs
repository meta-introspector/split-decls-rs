// Generated macro for run_thin (function)
macro_rules! Depcrate_back_ltorun_thin {
() => {
// Module: crate::back::lto
// Provides: {"run_thin"}
// Dependencies: {}
# [doc = " Performs thin LTO by performing necessary global analysis and returning two"] # [doc = " lists, one of the modules that need optimization and another for modules that"] # [doc = " can simply be copied over from the incr. comp. cache."] pub (crate) fn run_thin (cgcx : & CodegenContext < GccCodegenBackend > , each_linked_rlib_for_lto : & [PathBuf] , modules : Vec < (String , ThinBuffer) > , cached_modules : Vec < (SerializedModule < ModuleBuffer > , WorkProduct) > ,) -> (Vec < ThinModule < GccCodegenBackend > > , Vec < WorkProduct >) { let dcx = cgcx . create_dcx () ; let dcx = dcx . handle () ; let lto_data = prepare_lto (cgcx , each_linked_rlib_for_lto , dcx) ; if cgcx . opts . cg . linker_plugin_lto . enabled () { unreachable ! ("We should never reach this case if the LTO step \
                      is deferred to the linker") ; } thin_lto (cgcx , dcx , modules , lto_data . upstream_modules , lto_data . tmp_path , cached_modules ,) }
};
}
