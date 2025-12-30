// Generated macro for add_to_sysroot (function)
macro_rules! Depcrate_core_build_steps_compileadd_to_sysroot {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"add_to_sysroot"}
// Dependencies: {}
# [doc = " Link some files into a rustc sysroot."] # [doc = ""] # [doc = " For a particular stage this will link the file listed in `stamp` into the"] # [doc = " `sysroot_dst` provided."] # [track_caller] pub fn add_to_sysroot (builder : & Builder < '_ > , sysroot_dst : & Path , sysroot_host_dst : & Path , stamp : & BuildStamp ,) { let self_contained_dst = & sysroot_dst . join ("self-contained") ; t ! (fs :: create_dir_all (sysroot_dst)) ; t ! (fs :: create_dir_all (sysroot_host_dst)) ; t ! (fs :: create_dir_all (self_contained_dst)) ; for (path , dependency_type) in builder . read_stamp_file (stamp) { let dst = match dependency_type { DependencyType :: Host => sysroot_host_dst , DependencyType :: Target => sysroot_dst , DependencyType :: TargetSelfContained => self_contained_dst , } ; builder . copy_link (& path , & dst . join (path . file_name () . unwrap ()) , FileType :: Regular) ; } }
};
}
