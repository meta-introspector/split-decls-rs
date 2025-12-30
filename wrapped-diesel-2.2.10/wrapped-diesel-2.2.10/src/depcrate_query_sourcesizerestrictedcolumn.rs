// Generated macro for SizeRestrictedColumn (trait)
macro_rules! Depcrate_query_sourceSizeRestrictedColumn {
() => {
// Module: crate::query_source
// Provides: {"SizeRestrictedColumn"}
// Dependencies: {}
# [doc = " Max length for columns of type Char/Varchar..."] # [doc = ""] # [doc = " If a given column has a such constraint, this trait will be implemented and specify that"] # [doc = " length."] pub trait SizeRestrictedColumn : Column { # [doc = " Max length of that column"] const MAX_LENGTH : usize ; }
};
}
