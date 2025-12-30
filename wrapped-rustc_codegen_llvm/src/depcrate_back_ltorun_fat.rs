// Generated macro for run_fat (function)
macro_rules! Depcrate_back_ltorun_fat {
() => {
// Module: crate::back::lto
// Provides: {"run_fat"}
// Dependencies: {}
# [doc = " Performs fat LTO by merging all modules into a single one and returning it"] # [doc = " for further optimization."] pub (crate) fn run_fat (cgcx : & CodegenContext < LlvmCodegenBackend > , exported_symbols_for_lto : & [String] , each_linked_rlib_for_lto : & [PathBuf] , modules : Vec < FatLtoInput < LlvmCodegenBackend > > ,) -> ModuleCodegen < ModuleLlvm > { let dcx = cgcx . create_dcx () ; let dcx = dcx . handle () ; let (symbols_below_threshold , upstream_modules) = prepare_lto (cgcx , exported_symbols_for_lto , each_linked_rlib_for_lto , dcx) ; let symbols_below_threshold = symbols_below_threshold . iter () . map (| c | c . as_ptr ()) . collect :: < Vec < _ > > () ; fat_lto (cgcx , dcx , modules , upstream_modules , & symbols_below_threshold) }
};
}
