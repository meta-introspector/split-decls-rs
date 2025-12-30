// Generated macro for macro_721 (macro)
macro_rules! Depcrate_cargomacro_721 {
() => {
// Module: crate::cargo
// Provides: {"macro_721"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for negative feature names with prefix `no-` or `not-`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Features are supposed to be additive, and negatively-named features violate it."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```toml"] # [doc = " # The `Cargo.toml` with negative feature names"] # [doc = " [features]"] # [doc = " default = []"] # [doc = " no-abc = []"] # [doc = " not-def = []"] # [doc = ""] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```toml"] # [doc = " [features]"] # [doc = " default = [\"abc\", \"def\"]"] # [doc = " abc = []"] # [doc = " def = []"] # [doc = ""] # [doc = " ```"] # [clippy :: version = "1.57.0"] pub NEGATIVE_FEATURE_NAMES , cargo , "usage of a negative feature name" }
};
}
