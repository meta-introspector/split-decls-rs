// Generated macro for impl_1164 (impl)
macro_rules! Depcrate_units_converterimpl_1164 {
() => {
// Module: crate::units::converter
// Provides: {"impl_1164"}
// Dependencies: {}
impl < N > OffsetConverter < N > where N : Convertible , { # [doc = " Converts the given value from the input unit to the output unit."] pub (crate) fn convert (& self , value : & N) -> N { self . proportional . convert (value) . add_refs (& self . offset) } }
};
}
