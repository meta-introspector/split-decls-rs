// Generated macro for impl_1162 (impl)
macro_rules! Depcrate_units_converterimpl_1162 {
() => {
// Module: crate::units::converter
// Provides: {"impl_1162"}
// Dependencies: {}
impl < N > ReciprocalConverter < N > where N : Convertible , { # [doc = " Converts the given value from the input unit to the output unit."] pub (crate) fn convert (& self , value : & N) -> N { self . proportional . convert (value) . reciprocal () } }
};
}
