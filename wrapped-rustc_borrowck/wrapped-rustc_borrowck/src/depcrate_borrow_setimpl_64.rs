// Generated macro for impl_64 (impl)
macro_rules! Depcrate_borrow_setimpl_64 {
() => {
// Module: crate::borrow_set
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'tcx > Index < BorrowIndex > for BorrowSet < 'tcx > { type Output = BorrowData < 'tcx > ; fn index (& self , index : BorrowIndex) -> & BorrowData < 'tcx > { & self . location_map [index . as_usize ()] } }
};
}
