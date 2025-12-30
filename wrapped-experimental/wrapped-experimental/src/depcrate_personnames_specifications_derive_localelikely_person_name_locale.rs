// Generated macro for likely_person_name_locale (function)
macro_rules! Depcrate_personnames_specifications_derive_localelikely_person_name_locale {
() => {
// Module: crate::personnames::specifications::derive_locale
// Provides: {"likely_person_name_locale"}
// Dependencies: {}
# [doc = " https://www.unicode.org/reports/tr35/tr35-personNames.html#derive-the-name-locale"] pub fn likely_person_name_locale < N > (person_name : & N , swe : ScriptWithExtensionsBorrowed , scripts : PropertyNamesShortBorrowed < Script > ,) -> Result < Locale , PersonNamesFormatterError > where N : PersonName , { let mut found_name_script = find_script (person_name , swe , Surname) ; if found_name_script . is_none () { found_name_script = find_script (person_name , swe , Given) ; } let name_script = found_name_script . unwrap_or (icu_properties :: props :: Script :: Unknown) ; let locid_script = scripts . get_locale_script (name_script) . unwrap () ; person_name . name_locale () . map_or_else (| | { let mut effective_locale = Locale :: UNKNOWN ; effective_locale . id . script = Some (locid_script) ; Ok (effective_locale) } , | locale | { let mut effective_locale = locale . clone () ; effective_locale . id . script = Some (locid_script) ; Ok (effective_locale) } ,) }
};
}
