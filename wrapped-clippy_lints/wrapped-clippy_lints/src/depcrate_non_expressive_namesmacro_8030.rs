// Generated macro for macro_8030 (macro)
macro_rules! Depcrate_non_expressive_namesmacro_8030 {
() => {
// Module: crate::non_expressive_names
// Provides: {"macro_8030"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for names that are very similar and thus confusing."] # [doc = ""] # [doc = " Note: this lint looks for similar names throughout each"] # [doc = " scope. To allow it, you need to allow it on the scope"] # [doc = " level, not on the name that is reported."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's hard to distinguish between names that differ only"] # [doc = " by a single character."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " let checked_exp = something;"] # [doc = " let checked_expr = something_else;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub SIMILAR_NAMES , pedantic , "similarly named items and bindings" }
};
}
