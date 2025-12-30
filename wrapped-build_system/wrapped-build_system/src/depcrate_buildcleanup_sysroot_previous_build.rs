// Generated macro for cleanup_sysroot_previous_build (function)
macro_rules! Depcrate_buildcleanup_sysroot_previous_build {
() => {
// Module: crate::build
// Provides: {"cleanup_sysroot_previous_build"}
// Dependencies: {}
fn cleanup_sysroot_previous_build (library_dir : & Path) { let _ = walk_dir (library_dir . join ("target") , & mut | dir : & Path | { for top in & ["debug" , "release"] { let _ = fs :: remove_dir_all (dir . join (top) . join ("build")) ; let _ = fs :: remove_dir_all (dir . join (top) . join ("deps")) ; let _ = fs :: remove_dir_all (dir . join (top) . join ("examples")) ; let _ = fs :: remove_dir_all (dir . join (top) . join ("native")) ; let _ = walk_dir (dir . join (top) , & mut | sub_dir : & Path | { if sub_dir . file_name () . map (| filename | filename . to_str () . unwrap () . starts_with ("libsysroot")) . unwrap_or (false) { let _ = fs :: remove_dir_all (sub_dir) ; } Ok (()) } , & mut | file : & Path | { if file . file_name () . map (| filename | filename . to_str () . unwrap () . starts_with ("libsysroot")) . unwrap_or (false) { let _ = fs :: remove_file (file) ; } Ok (()) } , false ,) ; } Ok (()) } , & mut | _ | Ok (()) , false ,) ; }
};
}
