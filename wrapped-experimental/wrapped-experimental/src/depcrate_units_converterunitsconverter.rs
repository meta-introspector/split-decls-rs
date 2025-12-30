// Generated macro for UnitsConverter (struct)
macro_rules! Depcrate_units_converterUnitsConverter {
() => {
// Module: crate::units::converter
// Provides: {"UnitsConverter"}
// Dependencies: {}
# [doc = " A converter for converting between two single or compound units."] # [doc = " For example:"] # [doc = "     1 - `meter` to `foot`"] # [doc = "     2 - `mile-per-gallon` to `liter-per-100-kilometer`."] # [doc = "     3 - `celsius` to `fahrenheit`."] # [doc = ""] # [doc = " NOTE:"] # [doc = "     This converter does not support conversions between mixed units,"] # [doc = "     for example, from \"meter\" to \"foot-and-inch\"."] # [derive (Debug , Clone)] pub struct UnitsConverter < N > (pub (crate) UnitsConverterInner < N >) where N : Convertible ;
};
}
