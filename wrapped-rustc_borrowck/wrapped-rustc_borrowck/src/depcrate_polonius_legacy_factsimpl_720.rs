// Generated macro for impl_720 (impl)
macro_rules! Depcrate_polonius_legacy_factsimpl_720 {
() => {
// Module: crate::polonius::legacy::facts
// Provides: {"impl_720"}
// Dependencies: {}
impl FactRow for PoloniusRegionVid { fn write (& self , out : & mut dyn Write , location_table : & PoloniusLocationTable ,) -> Result < () , Box < dyn Error > > { write_row (out , location_table , & [self]) } }
};
}
