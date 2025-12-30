// Generated macro for pretty_print_float (function)
macro_rules! Depcrate_data_floatpretty_print_float {
() => {
// Module: crate::data::float
// Provides: {"pretty_print_float"}
// Dependencies: {}
# [doc = " The function that pretty prints the floating number"] # [doc = " Since rust doesn't have anything that can format a float with out appearance, so we just"] # [doc = " implement a float pretty printing function, which finds the shortest representation of a"] # [doc = " floating point number within the allowed error range."] # [doc = ""] # [doc = " - `n`: The float number to pretty-print"] # [doc = " - `allow_sn`: Should we use scientific notation when possible"] # [doc = " - **returns**: The pretty printed string"] pub fn pretty_print_float (n : f64 , allow_sn : bool) -> String { (FloatPrettyPrinter { allow_scientific : allow_sn , min_decimal : 0 , max_decimal : 10 , }) . print (n) }
};
}
