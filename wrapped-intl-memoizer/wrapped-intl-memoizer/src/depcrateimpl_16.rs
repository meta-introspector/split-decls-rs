// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl IntlMemoizer { # [doc = " Get a [`IntlLangMemoizer`] for a given language. If one does not exist for"] # [doc = " a locale, it will be constructed and weakly retained. See [`IntlLangMemoizer`]"] # [doc = " for more detailed documentation how to use it."] pub fn get_for_lang (& mut self , lang : LanguageIdentifier) -> Rc < IntlLangMemoizer > { match self . map . entry (lang . clone ()) { Entry :: Vacant (empty) => { let entry = Rc :: new (IntlLangMemoizer :: new (lang)) ; empty . insert (Rc :: downgrade (& entry)) ; entry } Entry :: Occupied (mut entry) => { if let Some (entry) = entry . get () . upgrade () { entry } else { let e = Rc :: new (IntlLangMemoizer :: new (lang)) ; entry . insert (Rc :: downgrade (& e)) ; e } } } } }
};
}
