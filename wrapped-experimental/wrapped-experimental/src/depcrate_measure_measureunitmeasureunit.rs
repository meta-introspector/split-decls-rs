// Generated macro for MeasureUnit (struct)
macro_rules! Depcrate_measure_measureunitMeasureUnit {
() => {
// Module: crate::measure::measureunit
// Provides: {"MeasureUnit"}
// Dependencies: {}
# [doc = " The [`MeasureUnit`] struct represents a processed CLDR compound unit."] # [doc = " Examples include:"] # [doc = "  1. `meter-per-second`"] # [doc = "  2. `square-meter`"] # [doc = "  3. `liter-per-100-kilometer`"] # [doc = "  4. `portion-per-1e9`"] # [doc = "  5. `square-meter` (Note: a single unit is a special case of a compound unit containing only one single unit.)"] # [derive (Debug , Eq , Clone)] pub struct MeasureUnit { # [doc = " The CLDR ID of the unit."] pub id : Option < & 'static str > , # [doc = " Contains the processed units."] pub (crate) single_units : SingleUnitVec , # [doc = " Represents the constant denominator of this measure unit."] # [doc = ""] # [doc = " Examples:"] # [doc = "   - For the unit `meter-per-second`, the constant denominator is `0`, because there is no denominator."] # [doc = "   - For the unit `liter-per-100-kilometer`, the constant denominator is `100`."] # [doc = "   - For the unit `portion-per-1e9`, the constant denominator is `1_000_000_000`."] # [doc = ""] # [doc = " NOTE:"] # [doc = "   If the constant denominator is not set, the value defaults to `0`."] pub (crate) constant_denominator : u64 , }
};
}
