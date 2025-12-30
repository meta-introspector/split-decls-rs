// Generated macro for Context (struct)
macro_rules! Depcrate_hoursContext {
() => {
// Module: crate::hours
// Provides: {"Context"}
// Dependencies: {}
# [doc = " Additional configuration for the hours estimation functionality."] pub struct Context < W > { # [doc = " Ignore github bots which match the `[bot]` search string."] pub ignore_bots : bool , # [doc = " Show personally identifiable information before the summary. Includes names and email addresses."] pub show_pii : bool , # [doc = " Collect how many files have been added, removed and modified (without rename tracking)."] pub file_stats : bool , # [doc = " Collect how many lines in files have been added, removed and modified (without rename tracking)."] pub line_stats : bool , # [doc = " The number of threads to use. If unset, use all cores, if 0 use all physical cores."] pub threads : Option < usize > , # [doc = " Omit unifying identities by name and email which can lead to the same author appear multiple times"] # [doc = " due to using different names or email addresses."] pub omit_unify_identities : bool , # [doc = " Where to write our output to"] pub out : W , }
};
}
