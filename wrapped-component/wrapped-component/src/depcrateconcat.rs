// Generated macro for Concat (function)
macro_rules! DepcrateConcat {
() => {
// Module: crate
// Provides: {"Concat"}
// Dependencies: {}
# [no_mangle] unsafe extern "system" fn Concat (left : PCWSTR , right : PCWSTR , result : * mut BSTR) -> HRESULT { if left . is_null () || right . is_null () || result . is_null () { return E_POINTER ; } unsafe { let left = left . as_wide () ; let right = right . as_wide () ; let combined : Vec < u16 > = [left , right] . concat () ; * result = BSTR :: from_wide (& combined) ; } S_OK }
};
}
