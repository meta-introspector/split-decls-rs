// Generated macro for impl_1902 (impl)
macro_rules! Depcrate_vec_in_place_dropimpl_1902 {
() => {
// Module: crate::vec::in_place_drop
// Provides: {"impl_1902"}
// Dependencies: {}
impl < T > Drop for InPlaceDrop < T > { # [inline] fn drop (& mut self) { unsafe { ptr :: drop_in_place (slice :: from_raw_parts_mut (self . inner , self . len ())) ; } } }
};
}
