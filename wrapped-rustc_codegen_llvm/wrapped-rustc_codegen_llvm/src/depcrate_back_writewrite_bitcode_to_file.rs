// Generated macro for write_bitcode_to_file (function)
macro_rules! Depcrate_back_writewrite_bitcode_to_file {
() => {
// Module: crate::back::write
// Provides: {"write_bitcode_to_file"}
// Dependencies: {}
fn write_bitcode_to_file (module : & ModuleCodegen < ModuleLlvm > , path : & Path) { unsafe { let path = path_to_c_string (& path) ; let llmod = module . module_llvm . llmod () ; llvm :: LLVMWriteBitcodeToFile (llmod , path . as_ptr ()) ; } }
};
}
