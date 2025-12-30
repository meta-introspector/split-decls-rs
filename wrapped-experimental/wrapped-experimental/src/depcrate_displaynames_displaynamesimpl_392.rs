// Generated macro for impl_392 (impl)
macro_rules! Depcrate_displaynames_displaynamesimpl_392 {
() => {
// Module: crate::displaynames::displaynames
// Provides: {"impl_392"}
// Dependencies: {}
impl RegionDisplayNames { icu_provider :: gen_buffer_data_constructors ! ((prefs : DisplayNamesPreferences , options : DisplayNamesOptions) -> error : DataError , # [doc = " Creates a new [`RegionDisplayNames`] from locale data and an options bag using compiled data."] functions : [try_new , try_new_with_buffer_provider , try_new_unstable , Self]) ; # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: try_new)] pub fn try_new_unstable < D : DataProvider < RegionDisplayNamesV1 > + ? Sized > (provider : & D , prefs : DisplayNamesPreferences , options : DisplayNamesOptions ,) -> Result < Self , DataError > { let locale = RegionDisplayNamesV1 :: make_locale (prefs . locale_preferences) ; let region_data = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_locale (& locale) , .. Default :: default () }) ? . payload ; Ok (Self { options , region_data , }) } # [doc = " Returns the display name of a region."] pub fn of (& self , region : Region) -> Option < & str > { let data = self . region_data . get () ; match self . options . style { Some (Style :: Short) => data . short_names . get (& region . to_tinystr () . to_unvalidated ()) , _ => None , } . or_else (| | data . names . get (& region . to_tinystr () . to_unvalidated ())) } }
};
}
