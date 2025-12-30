// Generated macro for demangle (function)
macro_rules! Depcrate_c_apidemangle {
() => {
// Module: crate::c_api
// Provides: {"demangle"}
// Dependencies: {}
# [no_mangle] pub unsafe extern "C" fn demangle (buffer : * const raw :: c_char , parse_options : cpp_demangle :: ParseOptions , demangle_options : cpp_demangle :: DemangleOptions ,) -> * mut raw :: c_char { if buffer . is_null () { return buffer as * mut raw :: c_char } let buffer = ffi :: CStr :: from_ptr (buffer) ; if let Ok ((symbol , _)) = cpp_demangle :: BorrowedSymbol :: with_tail_and_options (& buffer . to_bytes_with_nul () , & parse_options) { return ffi :: CString :: new (symbol . demangle_with_options (& demangle_options) . unwrap ()) . unwrap () . into_raw () ; } buffer . to_owned () . into_raw () }
};
}
