// Generated macro for impl_396 (impl)
macro_rules! Depcrate_displaynames_displaynamesimpl_396 {
() => {
// Module: crate::displaynames::displaynames
// Provides: {"impl_396"}
// Dependencies: {}
impl VariantDisplayNames { icu_provider :: gen_buffer_data_constructors ! ((prefs : DisplayNamesPreferences , options : DisplayNamesOptions) -> error : DataError , # [doc = " Creates a new [`VariantDisplayNames`] from locale data and an options bag using compiled data."] functions : [try_new , try_new_with_buffer_provider , try_new_unstable , Self]) ; # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: try_new)] pub fn try_new_unstable < D : DataProvider < VariantDisplayNamesV1 > + ? Sized > (provider : & D , prefs : DisplayNamesPreferences , options : DisplayNamesOptions ,) -> Result < Self , DataError > { let locale = VariantDisplayNamesV1 :: make_locale (prefs . locale_preferences) ; let variant_data = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_locale (& locale) , .. Default :: default () }) ? . payload ; Ok (Self { options , variant_data , }) } # [doc = " Returns the display name of a variant."] pub fn of (& self , variant : Variant) -> Option < & str > { let data = self . variant_data . get () ; data . names . get (& variant . to_tinystr () . to_unvalidated ()) } }
};
}
