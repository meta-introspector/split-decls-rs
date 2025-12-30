// Generated macro for is_mingw_gnu_toolchain (function)
macro_rules! Depcrate_commonis_mingw_gnu_toolchain {
() => {
// Module: crate::common
// Provides: {"is_mingw_gnu_toolchain"}
// Dependencies: {}
pub fn is_mingw_gnu_toolchain (target : & Target) -> bool { target . vendor == "pc" && target . os == "windows" && target . env == "gnu" && target . abi . is_empty () }
};
}
