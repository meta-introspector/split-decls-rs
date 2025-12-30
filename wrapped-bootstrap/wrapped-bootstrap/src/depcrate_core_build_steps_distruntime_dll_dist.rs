// Generated macro for runtime_dll_dist (function)
macro_rules! Depcrate_core_build_steps_distruntime_dll_dist {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"runtime_dll_dist"}
// Dependencies: {}
fn runtime_dll_dist (rust_root : & Path , target : TargetSelection , builder : & Builder < '_ >) { if builder . config . dry_run () { return ; } let (bin_path , libs_path) = get_cc_search_dirs (target , builder) ; let mut rustc_dlls = vec ! [] ; if target . ends_with ("windows-gnu") { rustc_dlls . push ("libwinpthread-1.dll") ; if target . starts_with ("i686-") { rustc_dlls . push ("libgcc_s_dw2-1.dll") ; } else { rustc_dlls . push ("libgcc_s_seh-1.dll") ; } } else if target . ends_with ("windows-gnullvm") { rustc_dlls . push ("libunwind.dll") ; } else { panic ! ("Vendoring of runtime DLLs for `{target}` is not supported`") ; } let bin_path = if target . ends_with ("windows-gnullvm") && builder . host_target != target { bin_path . into_iter () . chain (libs_path . iter () . map (| path | path . with_file_name ("bin"))) . collect () } else { bin_path } ; let rustc_dlls = find_files (& rustc_dlls , & bin_path) ; let rust_bin_dir = rust_root . join ("bin/") ; fs :: create_dir_all (& rust_bin_dir) . expect ("creating rust_bin_dir failed") ; for src in & rustc_dlls { builder . copy_link_to_folder (src , & rust_bin_dir) ; } if builder . config . lld_enabled { let rust_target_bin_dir = rust_root . join ("lib/rustlib") . join (target) . join ("bin") ; fs :: create_dir_all (& rust_target_bin_dir) . expect ("creating rust_target_bin_dir failed") ; for src in & rustc_dlls { builder . copy_link_to_folder (src , & rust_target_bin_dir) ; } } }
};
}
