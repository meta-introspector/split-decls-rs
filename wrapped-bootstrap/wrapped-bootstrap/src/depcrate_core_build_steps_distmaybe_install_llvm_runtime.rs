// Generated macro for maybe_install_llvm_runtime (function)
macro_rules! Depcrate_core_build_steps_distmaybe_install_llvm_runtime {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"maybe_install_llvm_runtime"}
// Dependencies: {}
# [doc = " Maybe add libLLVM.so to the runtime lib-dir for rustc itself."] # [cfg_attr (feature = "tracing" , instrument (level = "trace" , name = "maybe_install_llvm_runtime" , skip_all , fields (llvm_link_shared = ? builder . llvm_link_shared () , target = ? target , sysroot = ? sysroot ,) ,) ,)] pub fn maybe_install_llvm_runtime (builder : & Builder < '_ > , target : TargetSelection , sysroot : & Path) { let dst_libdir = sysroot . join (builder . sysroot_libdir_relative (Compiler :: new (1 , target))) ; if builder . llvm_link_shared () { maybe_install_llvm (builder , target , & dst_libdir , false) ; } }
};
}
