// Generated macro for impl_172 (impl)
macro_rules! Depcrate_columnimpl_172 {
() => {
// Module: crate::column
// Provides: {"impl_172"}
// Dependencies: {}
# [cfg (feature = "column_metadata")] impl ColumnMetadata < '_ > { # [inline] # [must_use] # [doc = " Returns the name of the column in the query results"] pub fn name (& self) -> & str { self . name } # [inline] # [must_use] # [doc = " Returns the database name from which the column originates"] pub fn database_name (& self) -> Option < & str > { self . database_name } # [inline] # [must_use] # [doc = " Returns the table name from which the column originates"] pub fn table_name (& self) -> Option < & str > { self . table_name } # [inline] # [must_use] # [doc = " Returns the column name from which the column originates"] pub fn origin_name (& self) -> Option < & str > { self . origin_name } }
};
}
