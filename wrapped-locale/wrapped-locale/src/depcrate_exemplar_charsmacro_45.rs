// Generated macro for macro_45 (macro)
macro_rules! Depcrate_exemplar_charsmacro_45 {
() => {
// Module: crate::exemplar_chars
// Provides: {"macro_45"}
// Dependencies: {}
make_exemplar_chars_unicode_set_property ! (dyn_data_marker : ExemplarCharactersIndex ; data_marker : LocaleExemplarCharactersIndexV1 ; func : pub fn try_new_index_unstable () ; # [doc = " Get the \"index\" set of exemplar characters."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::locale;"] # [doc = " use icu::locale::exemplar_chars::ExemplarCharacters;"] # [doc = ""] # [doc = " let exemplars_index ="] # [doc = "     ExemplarCharacters::try_new_index(&locale!(\"en\").into())"] # [doc = "     .expect(\"locale should be present\");"] # [doc = ""] # [doc = " assert!(!exemplars_index.contains('a'));"] # [doc = " assert!(!exemplars_index.contains('z'));"] # [doc = " assert!(!exemplars_index.contains_str(\"a\"));"] # [doc = " assert!(!exemplars_index.contains_str(\"ä\"));"] # [doc = " assert!(!exemplars_index.contains_str(\"ng\"));"] # [doc = " assert!(exemplars_index.contains_str(\"A\"));"] # [doc = " ```"] pub fn try_new_index () ;) ;
};
}
