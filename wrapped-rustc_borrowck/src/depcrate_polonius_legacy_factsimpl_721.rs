// Generated macro for impl_721 (impl)
macro_rules! Depcrate_polonius_legacy_factsimpl_721 {
() => {
// Module: crate::polonius::legacy::facts
// Provides: {"impl_721"}
// Dependencies: {}
impl < A , B > FactRow for (A , B) where A : FactCell , B : FactCell , { fn write (& self , out : & mut dyn Write , location_table : & PoloniusLocationTable ,) -> Result < () , Box < dyn Error > > { write_row (out , location_table , & [& self . 0 , & self . 1]) } }
};
}
