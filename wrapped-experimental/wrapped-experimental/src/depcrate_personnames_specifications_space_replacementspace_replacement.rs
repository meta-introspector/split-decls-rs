// Generated macro for space_replacement (function)
macro_rules! Depcrate_personnames_specifications_space_replacementspace_replacement {
() => {
// Module: crate::personnames::specifications::space_replacement
// Provides: {"space_replacement"}
// Dependencies: {}
# [doc = ""] # [doc = " https://www.unicode.org/reports/tr35/tr35-personNames.html#setting-the-spacereplacement"] pub fn space_replacement < 'lt > (formatting_locale : & Locale , person_name_locale : & Locale , foreign_space_replacement : Option < & 'lt str > ,) -> & 'lt str { let mut native_space_replacement = DEFAULT_FOREIGN_SPACE_REPLACEMENT ; if let Some (script) = formatting_locale . id . script { native_space_replacement = match script . as_str () { "Jpan" | "Hant" | "Hans" => "" , _ => DEFAULT_FOREIGN_SPACE_REPLACEMENT , } ; } if formatting_locale . id . language == person_name_locale . id . language { return native_space_replacement ; } foreign_space_replacement . unwrap_or (DEFAULT_FOREIGN_SPACE_REPLACEMENT) }
};
}
