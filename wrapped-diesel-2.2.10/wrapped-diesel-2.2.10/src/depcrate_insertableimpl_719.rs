// Generated macro for impl_719 (impl)
macro_rules! Depcrate_insertableimpl_719 {
() => {
// Module: crate::insertable
// Provides: {"impl_719"}
// Dependencies: {}
impl < T , DB > CanInsertInSingleQuery < DB > for & T where T : ? Sized + CanInsertInSingleQuery < DB > , DB : Backend , { fn rows_to_insert (& self) -> Option < usize > { (* self) . rows_to_insert () } }
};
}
