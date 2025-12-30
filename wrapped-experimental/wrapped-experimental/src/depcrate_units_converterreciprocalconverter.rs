// Generated macro for ReciprocalConverter (struct)
macro_rules! Depcrate_units_converterReciprocalConverter {
() => {
// Module: crate::units::converter
// Provides: {"ReciprocalConverter"}
// Dependencies: {}
# [doc = " A converter for converting between two units that are reciprocal."] # [doc = " For example:"] # [doc = "    1 - `meter-per-second` to `second-per-meter`."] # [doc = "    2 - `mile-per-gallon` to `liter-per-100-kilometer`."] # [derive (Debug , Clone)] pub (crate) struct ReciprocalConverter < N > where N : Convertible , { pub (crate) proportional : ProportionalConverter < N > , }
};
}
