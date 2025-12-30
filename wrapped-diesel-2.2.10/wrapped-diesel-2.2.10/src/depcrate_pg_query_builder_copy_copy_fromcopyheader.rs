// Generated macro for CopyHeader (enum)
macro_rules! Depcrate_pg_query_builder_copy_copy_fromCopyHeader {
() => {
// Module: crate::pg::query_builder::copy::copy_from
// Provides: {"CopyHeader"}
// Dependencies: {}
# [doc = " Describes the different possible settings for the `HEADER` option"] # [doc = " for `COPY FROM` statements"] # [derive (Debug , Copy , Clone)] pub enum CopyHeader { # [doc = " Is the header set?"] Set (bool) , # [doc = " Match the header with the targeted table names"] # [doc = " and fail in the case of a mismatch"] Match , }
};
}
