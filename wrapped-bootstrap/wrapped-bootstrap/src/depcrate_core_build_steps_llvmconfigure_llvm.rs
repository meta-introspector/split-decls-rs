// Generated macro for configure_llvm (function)
macro_rules! Depcrate_core_build_steps_llvmconfigure_llvm {
() => {
// Module: crate::core::build_steps::llvm
// Provides: {"configure_llvm"}
// Dependencies: {}
fn configure_llvm (builder : & Builder < '_ > , target : TargetSelection , cfg : & mut cmake :: Config) { if builder . config . llvm_thin_lto { cfg . define ("LLVM_ENABLE_LTO" , "Thin") ; if ! target . contains ("apple") { cfg . define ("LLVM_ENABLE_LLD" , "ON") ; } } if builder . config . llvm_libzstd { cfg . define ("LLVM_ENABLE_ZSTD" , "FORCE_ON") ; cfg . define ("LLVM_USE_STATIC_ZSTD" , "TRUE") ; } else { cfg . define ("LLVM_ENABLE_ZSTD" , "OFF") ; } if let Some (ref linker) = builder . config . llvm_use_linker { cfg . define ("LLVM_USE_LINKER" , linker) ; } if builder . config . llvm_allow_old_toolchain { cfg . define ("LLVM_TEMPORARILY_ALLOW_OLD_TOOLCHAIN" , "YES") ; } }
};
}
