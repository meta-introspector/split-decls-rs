macro_rules! deps {
    () => {
        LlvmCodegenBackend!();
        ThinBuffer!();
        ModuleBuffer!();
    };
}

macro_rules! run_thin {
    () => {
        deps!();
        # [doc = " Performs thin LTO by performing necessary global analysis and returning two"] # [doc = " lists, one of the modules that need optimization and another for modules that"] # [doc = " can simply be copied over from the incr. comp. cache."] pub (crate) fn run_thin (cgcx : & CodegenContext < LlvmCodegenBackend > , exported_symbols_for_lto : & [String] , each_linked_rlib_for_lto : & [PathBuf] , modules : Vec < (String , ThinBuffer) > , cached_modules : Vec < (SerializedModule < ModuleBuffer > , WorkProduct) > ,) -> (Vec < ThinModule < LlvmCodegenBackend > > , Vec < WorkProduct >) { let dcx = cgcx . create_dcx () ; let dcx = dcx . handle () ; let (symbols_below_threshold , upstream_modules) = prepare_lto (cgcx , exported_symbols_for_lto , each_linked_rlib_for_lto , dcx) ; let symbols_below_threshold = symbols_below_threshold . iter () . map (| c | c . as_ptr ()) . collect :: < Vec < _ > > () ; if cgcx . opts . cg . linker_plugin_lto . enabled () { unreachable ! ("We should never reach this case if the LTO step \
                      is deferred to the linker") ; } thin_lto (cgcx , dcx , modules , upstream_modules , cached_modules , & symbols_below_threshold) }
    };
}

run_thin!();