// Generated macro for impl_11 (impl)
macro_rules! Depcrate_rt_locationimpl_11 {
() => {
// Module: crate::rt::location
// Provides: {"impl_11"}
// Dependencies: {}
impl LocationSet { pub (super) fn new () -> LocationSet { LocationSet { locations : Default :: default () , } } pub (super) fn track (& mut self , location : Location , threads : & thread :: Set) { let active_id = threads . active_id () ; self . locations [active_id . as_usize ()] = location ; } }
};
}
