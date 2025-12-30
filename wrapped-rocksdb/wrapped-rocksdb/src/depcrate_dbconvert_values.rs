// Generated macro for convert_values (function)
macro_rules! Depcrate_dbconvert_values {
() => {
// Module: crate::db
// Provides: {"convert_values"}
// Dependencies: {}
pub (crate) fn convert_values (values : Vec < * mut c_char > , values_sizes : Vec < usize > , errors : Vec < * mut c_char > ,) -> Vec < Result < Option < Vec < u8 > > , Error > > { values . into_iter () . zip (values_sizes) . zip (errors) . map (| ((v , s) , e) | { if e . is_null () { let value = unsafe { crate :: ffi_util :: raw_data (v , s) } ; unsafe { ffi :: rocksdb_free (v as * mut c_void) ; } Ok (value) } else { Err (Error :: new (crate :: ffi_util :: error_message (e))) } }) . collect () }
};
}
