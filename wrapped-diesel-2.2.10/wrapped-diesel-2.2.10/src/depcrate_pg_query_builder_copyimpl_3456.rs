// Generated macro for impl_3456 (impl)
macro_rules! Depcrate_pg_query_builder_copyimpl_3456 {
() => {
// Module: crate::pg::query_builder::copy
// Provides: {"impl_3456"}
// Dependencies: {}
impl CopyFormat { fn to_sql_format (self) -> & 'static str { match self { CopyFormat :: Text => "text" , CopyFormat :: Csv => "csv" , CopyFormat :: Binary => "binary" , } } }
};
}
