// Generated macro for SI_PREFIXES_OFFSET (const)
macro_rules! Depcrate_measure_parser_si_prefixSI_PREFIXES_OFFSET {
() => {
// Module: crate::measure::parser::si_prefix
// Provides: {"SI_PREFIXES_OFFSET"}
// Dependencies: {}
# [doc = " The offset of the SI prefixes."] # [doc = " NOTE:"] # [doc = "     The offset is added to the power of the decimal SI prefixes in order to avoid negative powers."] # [doc = "     Therefore, if there is a prefix with power more than -30, the offset should be increased."] const SI_PREFIXES_OFFSET : u8 = 30 ;
};
}
