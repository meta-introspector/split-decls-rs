// Generated macro for impl_11 (impl)
macro_rules! Depcrate_concurrentimpl_11 {
() => {
// Module: crate::concurrent
// Provides: {"impl_11"}
// Dependencies: {}
impl IntlLangMemoizer { # [doc = " Create a new [`IntlLangMemoizer`] that is unique to a specific [`LanguageIdentifier`]"] pub fn new (lang : LanguageIdentifier) -> Self { Self { lang , map : Mutex :: new (type_map :: concurrent :: TypeMap :: new ()) , } } # [doc = " Lazily initialize and run a formatter. See"] # [doc = " [`intl_memoizer::IntlLangMemoizer::with_try_get`](crate::IntlLangMemoizer::with_try_get)"] # [doc = " for documentation."] pub fn with_try_get < I , R , U > (& self , args : I :: Args , cb : U) -> Result < R , I :: Error > where Self : Sized , I : Memoizable + Sync + Send + 'static , I :: Args : Send + Sync + 'static , U : FnOnce (& I) -> R , { let mut map = self . map . lock () . unwrap () ; let cache = map . entry :: < HashMap < I :: Args , I > > () . or_insert_with (HashMap :: new) ; let e = match cache . entry (args . clone ()) { Entry :: Occupied (entry) => entry . into_mut () , Entry :: Vacant (entry) => { let val = I :: construct (self . lang . clone () , args) ? ; entry . insert (val) } } ; Ok (cb (e)) } }
};
}
