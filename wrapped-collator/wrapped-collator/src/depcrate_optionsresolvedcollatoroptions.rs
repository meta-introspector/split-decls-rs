// Generated macro for ResolvedCollatorOptions (struct)
macro_rules! Depcrate_optionsResolvedCollatorOptions {
() => {
// Module: crate::options
// Provides: {"ResolvedCollatorOptions"}
// Dependencies: {}
# [doc = " The resolved (actually used) options used by the collator."] # [doc = ""] # [doc = " See the documentation of `CollatorOptions`."] # [non_exhaustive] # [derive (Debug , Copy , Clone)] pub struct ResolvedCollatorOptions { # [doc = " Resolved strength collation option."] pub strength : Strength , # [doc = " Resolved alternate handling collation option."] pub alternate_handling : AlternateHandling , # [doc = " Resolved case first collation option."] pub case_first : CollationCaseFirst , # [doc = " Resolved max variable collation option."] pub max_variable : MaxVariable , # [doc = " Resolved case level collation option."] pub case_level : CaseLevel , # [doc = " Resolved numeric collation option."] pub numeric : CollationNumericOrdering , }
};
}
