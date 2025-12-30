// Generated macro for macro_11257 (macro)
macro_rules! Depcrate_upper_case_acronymsmacro_11257 {
() => {
// Module: crate::upper_case_acronyms
// Provides: {"macro_11257"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for fully capitalized names and optionally names containing a capitalized acronym."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " In CamelCase, acronyms count as one word."] # [doc = " See [naming conventions](https://rust-lang.github.io/api-guidelines/naming.html#casing-conforms-to-rfc-430-c-case)"] # [doc = " for more."] # [doc = ""] # [doc = " By default, the lint only triggers on fully-capitalized names."] # [doc = " You can use the `upper-case-acronyms-aggressive: true` config option to enable linting"] # [doc = " on all camel case names"] # [doc = ""] # [doc = " ### Known problems"] # [doc = " When two acronyms are contiguous, the lint can't tell where"] # [doc = " the first acronym ends and the second starts, so it suggests to lowercase all of"] # [doc = " the letters in the second acronym."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct HTTPResponse;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " struct HttpResponse;"] # [doc = " ```"] # [clippy :: version = "1.51.0"] pub UPPER_CASE_ACRONYMS , style , "capitalized acronyms are against the naming convention" }
};
}
