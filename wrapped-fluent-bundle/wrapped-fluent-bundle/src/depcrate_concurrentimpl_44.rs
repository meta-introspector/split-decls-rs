// Generated macro for impl_44 (impl)
macro_rules! Depcrate_concurrentimpl_44 {
() => {
// Module: crate::concurrent
// Provides: {"impl_44"}
// Dependencies: {}
impl < R > FluentBundle < R > { # [doc = " A constructor analogous to [`FluentBundle::new`] but operating"] # [doc = " on a concurrent version of [`IntlLangMemoizer`] over [`Mutex`](std::sync::Mutex)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use fluent_bundle::concurrent::FluentBundle;"] # [doc = " use fluent_bundle::FluentResource;"] # [doc = " use unic_langid::langid;"] # [doc = ""] # [doc = " let langid_en = langid!(\"en-US\");"] # [doc = " let mut bundle: FluentBundle<FluentResource> ="] # [doc = "     FluentBundle::new_concurrent(vec![langid_en]);"] # [doc = " ```"] pub fn new_concurrent (locales : Vec < LanguageIdentifier >) -> Self { let first_locale = locales . first () . cloned () . unwrap_or_default () ; Self { locales , resources : vec ! [] , entries : FxHashMap :: default () , intls : IntlLangMemoizer :: new (first_locale) , use_isolating : true , transform : None , formatter : None , } } }
};
}
