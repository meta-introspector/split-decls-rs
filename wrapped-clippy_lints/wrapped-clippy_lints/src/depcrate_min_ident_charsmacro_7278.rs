// Generated macro for macro_7278 (macro)
macro_rules! Depcrate_min_ident_charsmacro_7278 {
() => {
// Module: crate::min_ident_chars
// Provides: {"macro_7278"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for identifiers which consist of a single character (or fewer than the configured threshold)."] # [doc = ""] # [doc = " Note: This lint can be very noisy when enabled; it may be desirable to only enable it"] # [doc = " temporarily."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " To improve readability by requiring that every variable has a name more specific than a single letter can be."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " for m in movies {"] # [doc = "     let title = m.t;"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " for movie in movies {"] # [doc = "     let title = movie.title;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " ### Limitations"] # [doc = " Trait implementations which use the same function or parameter name as the trait declaration will"] # [doc = " not be warned about, even if the name is below the configured limit."] # [clippy :: version = "1.72.0"] pub MIN_IDENT_CHARS , restriction , "disallows idents that are too short" }
};
}
