// Generated macro for impl_3441 (impl)
macro_rules! Depcrate_pg_query_builder_copy_copy_toimpl_3441 {
() => {
// Module: crate::pg::query_builder::copy::copy_to
// Provides: {"impl_3441"}
// Dependencies: {}
# [cfg (feature = "postgres")] impl RowIndex < usize > for CopyRow < '_ > { fn idx (& self , idx : usize) -> Option < usize > { if idx < self . field_count () { Some (idx) } else { None } } }
};
}
