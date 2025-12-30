// Generated macro for RawStatement (struct)
macro_rules! Depcrate_raw_statementRawStatement {
() => {
// Module: crate::raw_statement
// Provides: {"RawStatement"}
// Dependencies: {}
# [derive (Debug)] pub struct RawStatement { ptr : * mut ffi :: sqlite3_stmt , cache : ParamIndexCache , # [cfg (feature = "cache")] statement_cache_key : Option < Arc < str > > , }
};
}
