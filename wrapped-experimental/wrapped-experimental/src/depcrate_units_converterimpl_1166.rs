// Generated macro for impl_1166 (impl)
macro_rules! Depcrate_units_converterimpl_1166 {
() => {
// Module: crate::units::converter
// Provides: {"impl_1166"}
// Dependencies: {}
impl < N > ProportionalConverter < N > where N : Convertible , { # [doc = " Converts the given value from the input unit to the output unit."] pub fn convert (& self , value : & N) -> N { value . mul_refs (& self . conversion_rate) } }
};
}
