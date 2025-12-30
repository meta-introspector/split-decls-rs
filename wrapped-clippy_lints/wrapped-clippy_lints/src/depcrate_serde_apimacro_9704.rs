// Generated macro for macro_9704 (macro)
macro_rules! Depcrate_serde_apimacro_9704 {
() => {
// Module: crate::serde_api
// Provides: {"macro_9704"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for misuses of the serde API."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Serde is very finicky about how its API should be"] # [doc = " used, but the type system can't be used to enforce it (yet?)."] # [doc = ""] # [doc = " ### Example"] # [doc = " Implementing `Visitor::visit_string` but not"] # [doc = " `Visitor::visit_str`."] # [clippy :: version = "pre 1.29.0"] pub SERDE_API_MISUSE , correctness , "various things that will negatively affect your serde experience" }
};
}
