// Generated macro for impl_218 (impl)
macro_rules! Depcrate_optionsimpl_218 {
() => {
// Module: crate::options
// Provides: {"impl_218"}
// Dependencies: {}
impl From < CollatorOptionsBitField > for ResolvedCollatorOptions { fn from (options : CollatorOptionsBitField) -> ResolvedCollatorOptions { Self { strength : options . strength () , alternate_handling : options . alternate_handling () , case_first : options . case_first () , max_variable : options . max_variable () , case_level : if options . case_level () { CaseLevel :: On } else { CaseLevel :: Off } , numeric : if options . numeric () { CollationNumericOrdering :: True } else { CollationNumericOrdering :: False } , } } }
};
}
