// Generated macro for build_mismatch_suggestion (function)
macro_rules! Depcrate_lifetime_syntaxbuild_mismatch_suggestion {
() => {
// Module: crate::lifetime_syntax
// Provides: {"build_mismatch_suggestion"}
// Dependencies: {}
fn build_mismatch_suggestion (lifetime_name : & str , infos : & [& Info < '_ >] ,) -> lints :: MismatchedLifetimeSyntaxesSuggestion { let lifetime_name = lifetime_name . to_owned () ; let suggestions = infos . iter () . map (| info | info . suggestion (& lifetime_name)) . collect () ; lints :: MismatchedLifetimeSyntaxesSuggestion :: Explicit { lifetime_name , suggestions , optional_alternative : false , } }
};
}
