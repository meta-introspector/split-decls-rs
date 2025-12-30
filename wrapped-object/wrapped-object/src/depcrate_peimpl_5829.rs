// Generated macro for impl_5829 (impl)
macro_rules! Depcrate_peimpl_5829 {
() => {
// Module: crate::pe
// Provides: {"impl_5829"}
// Dependencies: {}
impl ImageImportDescriptor { # [doc = " Tell whether this import descriptor is the null descriptor"] # [doc = " (used to mark the end of the iterator array in a PE)"] pub fn is_null (& self) -> bool { self . original_first_thunk . get (LE) == 0 && self . time_date_stamp . get (LE) == 0 && self . forwarder_chain . get (LE) == 0 && self . name . get (LE) == 0 && self . first_thunk . get (LE) == 0 } }
};
}
