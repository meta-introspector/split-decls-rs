// Generated macro for copy_llvm_libunwind (function)
macro_rules! Depcrate_core_build_steps_compilecopy_llvm_libunwind {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"copy_llvm_libunwind"}
// Dependencies: {}
fn copy_llvm_libunwind (builder : & Builder < '_ > , target : TargetSelection , libdir : & Path) -> PathBuf { let libunwind_path = builder . ensure (llvm :: Libunwind { target }) ; let libunwind_source = libunwind_path . join ("libunwind.a") ; let libunwind_target = libdir . join ("libunwind.a") ; builder . copy_link (& libunwind_source , & libunwind_target , FileType :: NativeLibrary) ; libunwind_target }
};
}
