// Generated macro for Column (struct)
macro_rules! Depcrate_columnColumn {
() => {
// Module: crate::column
// Provides: {"Column"}
// Dependencies: {}
# [doc = " Information about a column of a SQLite query."] # [cfg (feature = "column_decltype")] # [derive (Debug)] pub struct Column < 'stmt > { name : & 'stmt str , decl_type : Option < & 'stmt str > , }
};
}
