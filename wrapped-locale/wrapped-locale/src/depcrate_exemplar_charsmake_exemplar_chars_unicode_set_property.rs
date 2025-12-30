// Generated macro for make_exemplar_chars_unicode_set_property (macro)
macro_rules! Depcrate_exemplar_charsmake_exemplar_chars_unicode_set_property {
() => {
// Module: crate::exemplar_chars
// Provides: {"make_exemplar_chars_unicode_set_property"}
// Dependencies: {}
macro_rules ! make_exemplar_chars_unicode_set_property { (dyn_data_marker : $ d : ident ; data_marker : $ data_marker : ty ; func : pub fn $ unstable : ident () ; $ (# [$ attr : meta]) * pub fn $ compiled : ident () ;) => { impl ExemplarCharactersBorrowed <'static > { $ (# [$ attr]) * # [cfg (feature = "compiled_data")] # [inline] pub fn $ compiled (locale : & DataLocale ,) -> Result < Self , DataError > { Ok (ExemplarCharactersBorrowed { data : DataProvider ::<$ data_marker >:: load (& crate :: provider :: Baked , DataRequest { id : DataIdentifierBorrowed :: for_locale (locale) , .. Default :: default () }) ? . payload . get_static () . ok_or_else (|| DataError :: custom ("Baked provider didn't return static payload")) ? }) } } impl ExemplarCharacters { $ (# [$ attr]) * # [cfg (feature = "compiled_data")] pub fn $ compiled (locale : & DataLocale ,) -> Result < ExemplarCharactersBorrowed <'static >, DataError > { ExemplarCharactersBorrowed ::$ compiled (locale) } # [doc = concat ! ("A version of [`Self::" , stringify ! ($ compiled) , "()`] that uses custom data provided by a [`DataProvider`].")] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] pub fn $ unstable (provider : & (impl DataProvider <$ data_marker > + ? Sized) , locale : & DataLocale ,) -> Result < Self , DataError > { Ok (Self { data : provider . load (DataRequest { id : DataIdentifierBorrowed :: for_locale (locale) , .. Default :: default () }) ? . payload . cast () }) } } } }
};
}
