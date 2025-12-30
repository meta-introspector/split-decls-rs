// Generated macro for is_static_available (function)
macro_rules! Depcrateis_static_available {
() => {
// Module: crate
// Provides: {"is_static_available"}
// Dependencies: {}
# [doc = " System libraries should only be linked dynamically"] fn is_static_available (name : & str , system_roots : & [PathBuf] , dirs : & [PathBuf]) -> bool { let libnames = { let mut names = vec ! [format ! ("lib{}.a" , name)] ; if cfg ! (target_os = "windows") { names . push (format ! ("{}.lib" , name)) ; } names } ; dirs . iter () . any (| dir | { let library_exists = libnames . iter () . any (| libname | dir . join (& libname) . exists ()) ; library_exists && ! system_roots . iter () . any (| sys | dir . starts_with (sys)) }) }
};
}
