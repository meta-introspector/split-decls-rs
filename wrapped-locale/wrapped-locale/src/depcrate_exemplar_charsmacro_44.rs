// Generated macro for macro_44 (macro)
macro_rules! Depcrate_exemplar_charsmacro_44 {
() => {
// Module: crate::exemplar_chars
// Provides: {"macro_44"}
// Dependencies: {}
make_exemplar_chars_unicode_set_property ! (dyn_data_marker : ExemplarCharactersNumbers ; data_marker : LocaleExemplarCharactersNumbersV1 ; func : pub fn try_new_numbers_unstable () ; # [doc = " Get the \"numbers\" set of exemplar characters."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::locale;"] # [doc = " use icu::locale::exemplar_chars::ExemplarCharacters;"] # [doc = ""] # [doc = " let exemplars_numbers ="] # [doc = "     ExemplarCharacters::try_new_numbers(&locale!(\"en\").into())"] # [doc = "     .expect(\"locale should be present\");"] # [doc = ""] # [doc = " assert!(exemplars_numbers.contains('0'));"] # [doc = " assert!(exemplars_numbers.contains('9'));"] # [doc = " assert!(exemplars_numbers.contains('%'));"] # [doc = " assert!(exemplars_numbers.contains(','));"] # [doc = " assert!(exemplars_numbers.contains('.'));"] # [doc = " assert!(!exemplars_numbers.contains('!'));"] # [doc = " assert!(!exemplars_numbers.contains('?'));"] # [doc = " ```"] pub fn try_new_numbers () ;) ;
};
}
