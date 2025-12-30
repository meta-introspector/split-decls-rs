// Generated macro for is_llvm_mingw_wrapper (function)
macro_rules! Depcrateis_llvm_mingw_wrapper {
() => {
// Module: crate
// Provides: {"is_llvm_mingw_wrapper"}
// Dependencies: {}
fn is_llvm_mingw_wrapper (clang_path : & Path) -> bool { if let Some (filename) = clang_path . file_name () . and_then (| file_name | file_name . to_str ()) { filename . ends_with ("-w64-mingw32-clang") || filename . ends_with ("-w64-mingw32-clang++") } else { false } }
};
}
