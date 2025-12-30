// Generated macro for macro_50 (macro)
macro_rules! Depcrate_ruremacro_50 {
() => {
// Module: crate::rure
// Provides: {"macro_50"}
// Dependencies: {}
ffi_fn ! { fn rure_capture_name_index (re : * const Regex , name : * const c_char ,) -> i32 { let re = unsafe { &* re } ; let name = unsafe { CStr :: from_ptr (name) } ; let name = match name . to_str () { Err (_) => return - 1 , Ok (name) => name , } ; re . capture_names . get (name) . map (|& i | i) . unwrap_or (- 1) } }
};
}
