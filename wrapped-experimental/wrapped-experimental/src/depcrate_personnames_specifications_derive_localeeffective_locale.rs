// Generated macro for effective_locale (function)
macro_rules! Depcrate_personnames_specifications_derive_localeeffective_locale {
() => {
// Module: crate::personnames::specifications::derive_locale
// Provides: {"effective_locale"}
// Dependencies: {}
# [doc = " Override the formatting payload to use based on specification rules."] # [doc = ""] # [doc = " if name locale and formatting locale are incompatible, name locale takes precedence"] # [doc = " it should dynamically load the name locale formatter using the data_provider given in constructor."] # [doc = " https://www.unicode.org/reports/tr35/tr35-personNames.html#switch-the-formatting-locale-if-necessary"] # [doc = ""] # [doc = " The formatter locale and name locale must be maximized first."] pub fn effective_locale < 'a > (formatter_locale : & 'a Locale , person_name_locale : & 'a Locale ,) -> Result < & 'a Locale , PersonNamesFormatterError > { let name_script = person_name_locale . id . script . unwrap () ; let formatter_script = formatter_locale . id . script . unwrap () ; if ! compatible_scripts (name_script , formatter_script) { return Ok (person_name_locale) ; } Ok (formatter_locale) }
};
}
