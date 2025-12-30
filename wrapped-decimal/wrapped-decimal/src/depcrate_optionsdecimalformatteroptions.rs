// Generated macro for DecimalFormatterOptions (struct)
macro_rules! Depcrate_optionsDecimalFormatterOptions {
() => {
// Module: crate::options
// Provides: {"DecimalFormatterOptions"}
// Dependencies: {}
# [doc = " A bag of options defining how numbers will be formatted by"] # [doc = " [`DecimalFormatter`](crate::DecimalFormatter)."] # [derive (Debug , Eq , PartialEq , Clone , Copy , Default , Hash)] # [non_exhaustive] pub struct DecimalFormatterOptions { # [doc = " When to render grouping separators."] # [doc = ""] # [doc = " Default is [`GroupingStrategy::Auto`]"] pub grouping_strategy : Option < GroupingStrategy > , }
};
}
