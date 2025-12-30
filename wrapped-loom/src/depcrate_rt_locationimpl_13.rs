// Generated macro for impl_13 (impl)
macro_rules! Depcrate_rt_locationimpl_13 {
() => {
// Module: crate::rt::location
// Provides: {"impl_13"}
// Dependencies: {}
impl ops :: Index < & thread :: Set > for LocationSet { type Output = Location ; fn index (& self , threads : & thread :: Set) -> & Location { let active_id = threads . active_id () ; self . locations . index (active_id . as_usize ()) } }
};
}
