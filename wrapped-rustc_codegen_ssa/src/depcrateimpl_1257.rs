// Generated macro for impl_1257 (impl)
macro_rules! Depcrateimpl_1257 {
() => {
// Module: crate
// Provides: {"impl_1257"}
// Dependencies: {}
impl From < & cstore :: NativeLib > for NativeLib { fn from (lib : & cstore :: NativeLib) -> Self { NativeLib { kind : lib . kind , filename : lib . filename , name : lib . name , cfg : lib . cfg . clone () , verbatim : lib . verbatim . unwrap_or (false) , dll_imports : lib . dll_imports . clone () , } } }
};
}
