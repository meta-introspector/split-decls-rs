// Generated macro for aggregate_context (function)
macro_rules! Depcrate_functionsaggregate_context {
() => {
// Module: crate::functions
// Provides: {"aggregate_context"}
// Dependencies: {}
unsafe fn aggregate_context < A > (ctx : * mut sqlite3_context , bytes : usize) -> Option < * mut * mut A > { let pac = ffi :: sqlite3_aggregate_context (ctx , bytes as c_int) as * mut * mut A ; if pac . is_null () { return None ; } Some (pac) }
};
}
