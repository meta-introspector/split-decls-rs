// Generated macro for impl_556 (impl)
macro_rules! Depcrate_measure_categoryimpl_556 {
() => {
// Module: crate::measure::category
// Provides: {"impl_556"}
// Dependencies: {}
impl < T : MeasureUnitCategory > CategorizedMeasureUnit < T > { # [doc = " Returns the CLDR ID of the unit."] pub fn cldr_id (& self) -> & str { match self . unit . id { Some (id) => id , None => unimplemented ! () , } } }
};
}
