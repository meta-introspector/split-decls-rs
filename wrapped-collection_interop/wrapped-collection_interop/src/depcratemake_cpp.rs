// Generated macro for make_cpp (function)
macro_rules! Depcratemake_cpp {
() => {
// Module: crate
// Provides: {"make_cpp"}
// Dependencies: {}
pub fn make_cpp () -> Result < ITest > { unsafe extern "system" { fn make_cpp (test : * mut * mut std :: ffi :: c_void) -> HRESULT ; } unsafe { let mut test = None ; make_cpp (& mut test as * mut _ as * mut _) . ok () ? ; Type :: from_default (& test) } }
};
}
