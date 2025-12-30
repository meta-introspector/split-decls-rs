// Generated macro for macro_43 (macro)
macro_rules! Depcrate_exemplar_charsmacro_43 {
() => {
// Module: crate::exemplar_chars
// Provides: {"macro_43"}
// Dependencies: {}
make_exemplar_chars_unicode_set_property ! (dyn_data_marker : ExemplarCharactersPunctuation ; data_marker : LocaleExemplarCharactersPunctuationV1 ; func : pub fn try_new_punctuation_unstable () ; # [doc = " Get the \"punctuation\" set of exemplar characters."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::locale;"] # [doc = " use icu::locale::exemplar_chars::ExemplarCharacters;"] # [doc = ""] # [doc = " let exemplars_punctuation ="] # [doc = "     ExemplarCharacters::try_new_punctuation(&locale!(\"en\").into())"] # [doc = "     .expect(\"locale should be present\");"] # [doc = ""] # [doc = " assert!(!exemplars_punctuation.contains('0'));"] # [doc = " assert!(!exemplars_punctuation.contains('9'));"] # [doc = " assert!(!exemplars_punctuation.contains('%'));"] # [doc = " assert!(exemplars_punctuation.contains(','));"] # [doc = " assert!(exemplars_punctuation.contains('.'));"] # [doc = " assert!(exemplars_punctuation.contains('!'));"] # [doc = " assert!(exemplars_punctuation.contains('?'));"] # [doc = " ```"] pub fn try_new_punctuation () ;) ;
};
}
