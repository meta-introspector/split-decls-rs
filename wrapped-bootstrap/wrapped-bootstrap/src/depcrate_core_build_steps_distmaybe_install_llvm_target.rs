// Generated macro for maybe_install_llvm_target (function)
macro_rules! Depcrate_core_build_steps_distmaybe_install_llvm_target {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"maybe_install_llvm_target"}
// Dependencies: {}
# [doc = " Maybe add libLLVM.so to the target lib-dir for linking."] # [cfg_attr (feature = "tracing" , instrument (level = "trace" , name = "maybe_install_llvm_target" , skip_all , fields (llvm_link_shared = ? builder . llvm_link_shared () , target = ? target , sysroot = ? sysroot ,) ,) ,)] pub fn maybe_install_llvm_target (builder : & Builder < '_ > , target : TargetSelection , sysroot : & Path) { let dst_libdir = sysroot . join ("lib/rustlib") . join (target) . join ("lib") ; if builder . llvm_link_shared () { maybe_install_llvm (builder , target , & dst_libdir , false) ; } }
};
}
