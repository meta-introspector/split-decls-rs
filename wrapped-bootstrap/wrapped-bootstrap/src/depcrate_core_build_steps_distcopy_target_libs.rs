// Generated macro for copy_target_libs (function)
macro_rules! Depcrate_core_build_steps_distcopy_target_libs {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"copy_target_libs"}
// Dependencies: {}
# [doc = " Copy stamped files into an image's `target/lib` directory."] fn copy_target_libs (builder : & Builder < '_ > , target : TargetSelection , image : & Path , stamp : & BuildStamp ,) { let dst = image . join ("lib/rustlib") . join (target) . join ("lib") ; let self_contained_dst = dst . join ("self-contained") ; t ! (fs :: create_dir_all (& dst)) ; t ! (fs :: create_dir_all (& self_contained_dst)) ; for (path , dependency_type) in builder . read_stamp_file (stamp) { if dependency_type == DependencyType :: TargetSelfContained { builder . copy_link (& path , & self_contained_dst . join (path . file_name () . unwrap ()) , FileType :: NativeLibrary ,) ; } else if dependency_type == DependencyType :: Target || builder . config . is_host_target (target) { builder . copy_link (& path , & dst . join (path . file_name () . unwrap ()) , FileType :: NativeLibrary) ; } } }
};
}
