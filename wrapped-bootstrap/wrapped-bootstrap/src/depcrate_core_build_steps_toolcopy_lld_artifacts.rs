// Generated macro for copy_lld_artifacts (function)
macro_rules! Depcrate_core_build_steps_toolcopy_lld_artifacts {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"copy_lld_artifacts"}
// Dependencies: {}
pub (crate) fn copy_lld_artifacts (builder : & Builder < '_ > , lld_wrapper : BuiltLldWrapper , target_compiler : Compiler ,) { let target = target_compiler . host ; let libdir_bin = builder . sysroot_target_bindir (target_compiler , target) ; t ! (fs :: create_dir_all (& libdir_bin)) ; let src_exe = exe ("lld" , target) ; let dst_exe = exe ("rust-lld" , target) ; builder . copy_link (& lld_wrapper . lld_dir . join ("bin") . join (src_exe) , & libdir_bin . join (dst_exe) , FileType :: Executable ,) ; let self_contained_lld_dir = libdir_bin . join ("gcc-ld") ; t ! (fs :: create_dir_all (& self_contained_lld_dir)) ; for name in crate :: LLD_FILE_NAMES { builder . copy_link (& lld_wrapper . tool . tool_path , & self_contained_lld_dir . join (exe (name , target)) , FileType :: Executable ,) ; } }
};
}
