// Generated macro for print_error (function)
macro_rules! Depcrate_core_build_steps_toolstateprint_error {
() => {
// Module: crate::core::build_steps::toolstate
// Provides: {"print_error"}
// Dependencies: {}
fn print_error (tool : & str , submodule : & str) { eprintln ! () ; eprintln ! ("We detected that this PR updated '{tool}', but its tests failed.") ; eprintln ! () ; eprintln ! ("If you do intend to update '{tool}', please check the error messages above and") ; eprintln ! ("commit another update.") ; eprintln ! () ; eprintln ! ("If you do NOT intend to update '{tool}', please ensure you did not accidentally") ; eprintln ! ("change the submodule at '{submodule}'. You may ask your reviewer for the") ; eprintln ! ("proper steps.") ; crate :: exit ! (3) ; }
};
}
