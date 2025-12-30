// Generated macro for impl_720 (impl)
macro_rules! Depcrate_insertableimpl_720 {
() => {
// Module: crate::insertable
// Provides: {"impl_720"}
// Dependencies: {}
impl < T , U , DB > CanInsertInSingleQuery < DB > for ColumnInsertValue < T , U > where DB : Backend , { fn rows_to_insert (& self) -> Option < usize > { Some (1) } }
};
}
