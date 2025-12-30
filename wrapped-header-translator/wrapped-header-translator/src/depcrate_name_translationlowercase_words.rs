// Generated macro for lowercase_words (function)
macro_rules! Depcrate_name_translationlowercase_words {
() => {
// Module: crate::name_translation
// Provides: {"lowercase_words"}
// Dependencies: {}
fn lowercase_words (s : & str) -> impl Iterator < Item = String > + '_ { let mut has_seen_non_underscore = false ; split_words (s) . filter (move | word | { if * word == "_" { ! has_seen_non_underscore } else { has_seen_non_underscore = true ; true } }) . map (str :: to_lowercase) }
};
}
