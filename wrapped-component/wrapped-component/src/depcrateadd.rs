// Generated macro for Add (function)
macro_rules! DepcrateAdd {
() => {
// Module: crate
// Provides: {"Add"}
// Dependencies: {}
# [no_mangle] unsafe extern "system" fn Add (left : u32 , right : u32 , result : * mut u32) -> HRESULT { if result . is_null () { return E_POINTER ; } unsafe { * result = left + right ; } S_OK }
};
}
