// Generated macro for run_fat (function)
macro_rules! Depcrate_back_ltorun_fat {
() => {
// Module: crate::back::lto
// Provides: {"run_fat"}
// Dependencies: {}
# [doc = " Performs fat LTO by merging all modules into a single one and returning it"] # [doc = " for further optimization."] pub (crate) fn run_fat (cgcx : & CodegenContext < GccCodegenBackend > , each_linked_rlib_for_lto : & [PathBuf] , modules : Vec < FatLtoInput < GccCodegenBackend > > ,) -> ModuleCodegen < GccContext > { let dcx = cgcx . create_dcx () ; let dcx = dcx . handle () ; let lto_data = prepare_lto (cgcx , each_linked_rlib_for_lto , dcx) ; fat_lto (cgcx , dcx , modules , lto_data . upstream_modules , lto_data . tmp_path ,) }
};
}
