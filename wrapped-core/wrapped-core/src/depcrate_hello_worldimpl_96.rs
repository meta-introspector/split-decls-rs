// Generated macro for impl_96 (impl)
macro_rules! Depcrate_hello_worldimpl_96 {
() => {
// Module: crate::hello_world
// Provides: {"impl_96"}
// Dependencies: {}
impl HelloWorldFormatter { # [doc = " Creates a new [`HelloWorldFormatter`] for the specified locale."] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] pub fn try_new (prefs : HelloWorldFormatterPreferences) -> Result < Self , DataError > { Self :: try_new_unstable (& HelloWorldProvider , prefs) } icu_provider :: gen_buffer_data_constructors ! ((prefs : HelloWorldFormatterPreferences) -> error : DataError , functions : [try_new : skip , try_new_with_buffer_provider , try_new_unstable , Self ,]) ; # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: try_new)] pub fn try_new_unstable < P > (provider : & P , prefs : HelloWorldFormatterPreferences ,) -> Result < Self , DataError > where P : DataProvider < HelloWorldV1 > , { let locale = HelloWorldV1 :: make_locale (prefs . locale_preferences) ; let data = provider . load (DataRequest { id : crate :: request :: DataIdentifierBorrowed :: for_locale (& locale) , .. Default :: default () }) ? . payload ; Ok (Self { data }) } # [doc = " Formats a hello world message, returning a [`FormattedHelloWorld`]."] pub fn format < 'l > (& 'l self) -> FormattedHelloWorld < 'l > { FormattedHelloWorld { data : self . data . get () , } } # [doc = " Formats a hello world message, returning a [`String`]."] pub fn format_to_string (& self) -> String { self . format () . write_to_string () . into_owned () } }
};
}
