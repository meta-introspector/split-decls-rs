// Generated macro for impl_200 (impl)
macro_rules! Depcrate_dimension_percent_formatterimpl_200 {
() => {
// Module: crate::dimension::percent::formatter
// Provides: {"impl_200"}
// Dependencies: {}
impl < R > PercentFormatter < R > where R : AsRef < DecimalFormatter > , { # [doc = " Creates a new [`PercentFormatter`] from compiled locale data, an options bag and fixed decimal formatter."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] pub fn try_new_with_decimal_formatter (prefs : PercentFormatterPreferences , decimal_formatter : R , options : PercentFormatterOptions ,) -> Result < Self , DataError > { let locale = PercentEssentialsV1 :: make_locale (prefs . locale_preferences) ; let essential = crate :: provider :: Baked . load (DataRequest { id : DataIdentifierBorrowed :: for_locale (& locale) , .. Default :: default () }) ? . payload ; Ok (Self { essential , options , decimal_formatter , }) } # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: try_new)] pub fn try_new_with_decimal_formatter_unstable (provider : & (impl DataProvider < PercentEssentialsV1 > + ? Sized) , prefs : PercentFormatterPreferences , decimal_formatter : R , options : PercentFormatterOptions ,) -> Result < Self , DataError > { let locale = PercentEssentialsV1 :: make_locale (prefs . locale_preferences) ; let essential = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_locale (& locale) , .. Default :: default () }) ? . payload ; Ok (Self { essential , options , decimal_formatter , }) } }
};
}
