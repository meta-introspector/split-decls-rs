// Generated macro for ColumnMetadata (struct)
macro_rules! Depcrate_columnColumnMetadata {
() => {
// Module: crate::column
// Provides: {"ColumnMetadata"}
// Dependencies: {}
# [doc = " Metadata about the origin of a column of a SQLite query"] # [cfg (feature = "column_metadata")] # [derive (Debug)] pub struct ColumnMetadata < 'stmt > { name : & 'stmt str , database_name : Option < & 'stmt str > , table_name : Option < & 'stmt str > , origin_name : Option < & 'stmt str > , }
};
}
