// Generated macro for macro_41 (macro)
macro_rules! Depcrate_exemplar_charsmacro_41 {
() => {
// Module: crate::exemplar_chars
// Provides: {"macro_41"}
// Dependencies: {}
make_exemplar_chars_unicode_set_property ! (dyn_data_marker : ExemplarCharactersMain ; data_marker : LocaleExemplarCharactersMainV1 ; func : pub fn try_new_main_unstable () ; # [doc = " Get the \"main\" set of exemplar characters."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::locale;"] # [doc = " use icu::locale::exemplar_chars::ExemplarCharacters;"] # [doc = ""] # [doc = " let exemplars_main = ExemplarCharacters::try_new_main(&locale!(\"en\").into())"] # [doc = "     .expect(\"locale should be present\");"] # [doc = ""] # [doc = " assert!(exemplars_main.contains('a'));"] # [doc = " assert!(exemplars_main.contains('z'));"] # [doc = " assert!(exemplars_main.contains_str(\"a\"));"] # [doc = " assert!(!exemplars_main.contains_str(\"ä\"));"] # [doc = " assert!(!exemplars_main.contains_str(\"ng\"));"] # [doc = " assert!(!exemplars_main.contains_str(\"A\"));"] # [doc = " ```"] pub fn try_new_main () ;) ;
};
}
