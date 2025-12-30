// Generated macro for OffsetConverter (struct)
macro_rules! Depcrate_units_converterOffsetConverter {
() => {
// Module: crate::units::converter
// Provides: {"OffsetConverter"}
// Dependencies: {}
# [doc = " A converter for converting between two units that require an offset."] # [derive (Debug , Clone)] pub (crate) struct OffsetConverter < N > where N : Convertible , { # [doc = " The proportional converter."] pub (crate) proportional : ProportionalConverter < N > , # [doc = " The offset value to be added to the result of the proportional converter."] pub (crate) offset : N , }
};
}
