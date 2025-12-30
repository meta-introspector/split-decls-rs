// Generated macro for impl_722 (impl)
macro_rules! Depcrate_polonius_legacy_factsimpl_722 {
() => {
// Module: crate::polonius::legacy::facts
// Provides: {"impl_722"}
// Dependencies: {}
impl < A , B , C > FactRow for (A , B , C) where A : FactCell , B : FactCell , C : FactCell , { fn write (& self , out : & mut dyn Write , location_table : & PoloniusLocationTable ,) -> Result < () , Box < dyn Error > > { write_row (out , location_table , & [& self . 0 , & self . 1 , & self . 2]) } }
};
}
