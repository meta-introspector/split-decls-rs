// Generated macro for get_power (function)
macro_rules! Depcrate_measure_parser_powerget_power {
() => {
// Module: crate::measure::parser::power
// Provides: {"get_power"}
// Dependencies: {}
# [doc = " Extracts the power from the given CLDR ID part."] # [doc = "     - If the power is not found, the function returns (1, part)."] # [doc = "     - If the power is found, the function will return (power, part without the string of the power)."] pub fn get_power (part : & [u8]) -> (u8 , & [u8]) { let mut cursor = POWERS_TRIE . cursor () ; let mut longest_match = (1 , part) ; for (i , & b) in part . iter () . enumerate () { cursor . step (b) ; if cursor . is_empty () { break ; } if let Some (value) = cursor . take_value () { longest_match = (value as u8 , & part [i + 1 ..]) ; } } longest_match }
};
}
