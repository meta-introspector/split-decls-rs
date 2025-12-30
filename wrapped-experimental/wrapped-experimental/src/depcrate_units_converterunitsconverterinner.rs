// Generated macro for UnitsConverterInner (enum)
macro_rules! Depcrate_units_converterUnitsConverterInner {
() => {
// Module: crate::units::converter
// Provides: {"UnitsConverterInner"}
// Dependencies: {}
# [doc = " Enum containing all the of converters: Proportional, Reciprocal, and Offset converters as follows:"] # [doc = "    1 - Proportional: Converts between two units that are proportionally related (e.g. `meter` to `foot`)."] # [doc = "    2 - Reciprocal: Converts between two units that are reciprocal (e.g. `mile-per-gallon` to `liter-per-100-kilometer`)."] # [doc = "    3 - Offset: Converts between two units that require an offset (e.g. `celsius` to `fahrenheit`)."] # [derive (Debug , Clone)] pub (crate) enum UnitsConverterInner < N > where N : Convertible , { Proportional (ProportionalConverter < N >) , Reciprocal (ReciprocalConverter < N >) , Offset (OffsetConverter < N >) , }
};
}
