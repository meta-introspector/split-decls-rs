// Generated macro for impl_67 (impl)
macro_rules! Depcrate_borrow_setimpl_67 {
() => {
// Module: crate::borrow_set
// Provides: {"impl_67"}
// Dependencies: {}
impl < 'tcx > BorrowData < 'tcx > { pub fn reserve_location (& self) -> Location { self . reserve_location } pub fn activation_location (& self) -> TwoPhaseActivation { self . activation_location } pub fn kind (& self) -> mir :: BorrowKind { self . kind } pub fn region (& self) -> RegionVid { self . region } pub fn borrowed_place (& self) -> mir :: Place < 'tcx > { self . borrowed_place } pub fn assigned_place (& self) -> mir :: Place < 'tcx > { self . assigned_place } }
};
}
