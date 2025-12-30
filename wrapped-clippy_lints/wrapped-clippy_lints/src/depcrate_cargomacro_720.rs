// Generated macro for macro_720 (macro)
macro_rules! Depcrate_cargomacro_720 {
() => {
// Module: crate::cargo
// Provides: {"macro_720"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for feature names with prefix `use-`, `with-` or suffix `-support`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " These prefixes and suffixes have no significant meaning."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```toml"] # [doc = " # The `Cargo.toml` with feature name redundancy"] # [doc = " [features]"] # [doc = " default = [\"use-abc\", \"with-def\", \"ghi-support\"]"] # [doc = " use-abc = []  // redundant"] # [doc = " with-def = []   // redundant"] # [doc = " ghi-support = []   // redundant"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```toml"] # [doc = " [features]"] # [doc = " default = [\"abc\", \"def\", \"ghi\"]"] # [doc = " abc = []"] # [doc = " def = []"] # [doc = " ghi = []"] # [doc = " ```"] # [doc = ""] # [clippy :: version = "1.57.0"] pub REDUNDANT_FEATURE_NAMES , cargo , "usage of a redundant feature name" }
};
}
