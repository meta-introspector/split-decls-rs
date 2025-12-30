// Generated macro for ProportionalConverter (struct)
macro_rules! Depcrate_units_converterProportionalConverter {
() => {
// Module: crate::units::converter
// Provides: {"ProportionalConverter"}
// Dependencies: {}
# [doc = " ProportionalConverter is responsible for converting between two units that are proportionally related."] # [doc = " For example: 1- `meter` to `foot`."] # [doc = "              2- `square-meter` to `square-foot`."] # [doc = ""] # [doc = " However, it cannot convert between two units that are not proportionally related,"] # [doc = " such as `celsius` to `fahrenheit` and `mile-per-gallon` to `liter-per-100-kilometer`."] # [doc = ""] # [doc = " Also, it cannot convert between two units that are not single, such as `meter` to `foot-and-inch`."] # [derive (Debug , Clone)] pub (crate) struct ProportionalConverter < N > where N : Convertible , { # [doc = " The conversion rate between the input and output units."] pub (crate) conversion_rate : N , }
};
}
