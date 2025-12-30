// Generated macro for impl_251 (impl)
macro_rules! Depcrate_data_output_countimpl_251 {
() => {
// Module: crate::data::output::count
// Provides: {"impl_251"}
// Dependencies: {}
impl Count { # [doc = " Create a new instance from the given `oid` and its corresponding location."] pub fn from_data (oid : impl Into < ObjectId > , location : Option < crate :: data :: entry :: Location >) -> Self { Count { id : oid . into () , entry_pack_location : PackLocation :: LookedUp (location) , } } }
};
}
