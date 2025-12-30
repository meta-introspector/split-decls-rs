// Generated macro for get_si_prefix (function)
macro_rules! Depcrate_measure_parser_si_prefixget_si_prefix {
() => {
// Module: crate::measure::parser::si_prefix
// Provides: {"get_si_prefix"}
// Dependencies: {}
# [doc = " Extracts the SI prefix."] # [doc = " NOTE:"] # [doc = "    if the prefix is found, the function will return (SiPrefix, part without the prefix string)."] # [doc = "    if the prefix is not found, the function will return (SiPrefix { power: 0, base: Base::Decimal }, part)."] pub fn get_si_prefix (part : & [u8]) -> (SiPrefix , & [u8]) { let (si_prefix_base_10 , part) = get_si_prefix_base_ten (part) ; if si_prefix_base_10 != 0 { return (SiPrefix { power : si_prefix_base_10 , base : Base :: Decimal , } , part ,) ; } let (si_prefix_base_2 , part) = get_si_prefix_base_two (part) ; if si_prefix_base_2 != 0 { return (SiPrefix { power : si_prefix_base_2 , base : Base :: Binary , } , part ,) ; } (SiPrefix { power : 0 , base : Base :: Decimal , } , part ,) }
};
}
