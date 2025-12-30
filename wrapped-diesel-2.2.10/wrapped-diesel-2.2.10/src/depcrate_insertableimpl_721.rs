// Generated macro for impl_721 (impl)
macro_rules! Depcrate_insertableimpl_721 {
() => {
// Module: crate::insertable
// Provides: {"impl_721"}
// Dependencies: {}
impl < V , DB > CanInsertInSingleQuery < DB > for DefaultableColumnInsertValue < V > where DB : Backend , V : CanInsertInSingleQuery < DB > , { fn rows_to_insert (& self) -> Option < usize > { Some (1) } }
};
}
