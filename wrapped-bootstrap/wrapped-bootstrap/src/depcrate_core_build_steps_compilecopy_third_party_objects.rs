// Generated macro for copy_third_party_objects (function)
macro_rules! Depcrate_core_build_steps_compilecopy_third_party_objects {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"copy_third_party_objects"}
// Dependencies: {}
# [doc = " Copies third party objects needed by various targets."] fn copy_third_party_objects (builder : & Builder < '_ > , compiler : & Compiler , target : TargetSelection ,) -> Vec < (PathBuf , DependencyType) > { let mut target_deps = vec ! [] ; if builder . config . needs_sanitizer_runtime_built (target) && compiler . stage != 0 { target_deps . extend (copy_sanitizers (builder , compiler , target) . into_iter () . map (| d | (d , DependencyType :: Target)) ,) ; } if target == "x86_64-fortanix-unknown-sgx" || builder . config . llvm_libunwind (target) == LlvmLibunwind :: InTree && (target . contains ("linux") || target . contains ("fuchsia") || target . contains ("aix")) { let libunwind_path = copy_llvm_libunwind (builder , target , & builder . sysroot_target_libdir (* compiler , target)) ; target_deps . push ((libunwind_path , DependencyType :: Target)) ; } target_deps }
};
}
