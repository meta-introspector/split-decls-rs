// Generated macro for macro_42 (macro)
macro_rules! Depcrate_exemplar_charsmacro_42 {
() => {
// Module: crate::exemplar_chars
// Provides: {"macro_42"}
// Dependencies: {}
make_exemplar_chars_unicode_set_property ! (dyn_data_marker : ExemplarCharactersAuxiliary ; data_marker : LocaleExemplarCharactersAuxiliaryV1 ; func : pub fn try_new_auxiliary_unstable () ; # [doc = " Get the \"auxiliary\" set of exemplar characters."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::locale;"] # [doc = " use icu::locale::exemplar_chars::ExemplarCharacters;"] # [doc = ""] # [doc = " let exemplars_auxiliary ="] # [doc = "     ExemplarCharacters::try_new_auxiliary(&locale!(\"en\").into())"] # [doc = "     .expect(\"locale should be present\");"] # [doc = ""] # [doc = " assert!(!exemplars_auxiliary.contains('a'));"] # [doc = " assert!(!exemplars_auxiliary.contains('z'));"] # [doc = " assert!(!exemplars_auxiliary.contains_str(\"a\"));"] # [doc = " assert!(exemplars_auxiliary.contains_str(\"ä\"));"] # [doc = " assert!(!exemplars_auxiliary.contains_str(\"ng\"));"] # [doc = " assert!(!exemplars_auxiliary.contains_str(\"A\"));"] # [doc = " ```"] pub fn try_new_auxiliary () ;) ;
};
}
