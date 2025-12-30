// Generated macro for get_si_prefix_base_ten (function)
macro_rules! Depcrate_measure_parser_si_prefixget_si_prefix_base_ten {
() => {
// Module: crate::measure::parser::si_prefix
// Provides: {"get_si_prefix_base_ten"}
// Dependencies: {}
# [doc = " Extracts the SI prefix of base 10."] # [doc = " NOTE:"] # [doc = "    if the prefix is found, the function will return (power, part without the prefix)."] # [doc = "    if the prefix is not found, the function will return (0, part)."] fn get_si_prefix_base_ten (part : & [u8]) -> (i8 , & [u8]) { let mut cursor = DECIMAL_PREFIXES_TRIE . cursor () ; let mut longest_match = (0_i8 , part) ; for (i , & b) in part . iter () . enumerate () { cursor . step (b) ; if cursor . is_empty () { break ; } if let Some (value) = cursor . take_value () { let power = value as i8 - SI_PREFIXES_OFFSET as i8 ; longest_match = (power , & part [i + 1 ..]) ; } } longest_match }
};
}
