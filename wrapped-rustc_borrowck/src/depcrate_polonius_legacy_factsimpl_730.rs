// Generated macro for impl_730 (impl)
macro_rules! Depcrate_polonius_legacy_factsimpl_730 {
() => {
// Module: crate::polonius::legacy::facts
// Provides: {"impl_730"}
// Dependencies: {}
impl FactCell for LocationIndex { fn to_string (& self , location_table : & PoloniusLocationTable) -> String { format ! ("{:?}" , location_table . to_rich_location (* self)) } }
};
}
