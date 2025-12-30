// Generated macro for save_temp_bitcode (function)
macro_rules! Depcrate_back_writesave_temp_bitcode {
() => {
// Module: crate::back::write
// Provides: {"save_temp_bitcode"}
// Dependencies: {}
pub (crate) fn save_temp_bitcode (cgcx : & CodegenContext < LlvmCodegenBackend > , module : & ModuleCodegen < ModuleLlvm > , name : & str ,) { if ! cgcx . save_temps { return ; } let ext = format ! ("{name}.bc") ; let path = cgcx . output_filenames . temp_path_ext_for_cgu (& ext , & module . name , cgcx . invocation_temp . as_deref () ,) ; write_bitcode_to_file (module , & path) }
};
}
