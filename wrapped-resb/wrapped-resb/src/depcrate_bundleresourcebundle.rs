// Generated macro for ResourceBundle (struct)
macro_rules! Depcrate_bundleResourceBundle {
() => {
// Module: crate::bundle
// Provides: {"ResourceBundle"}
// Dependencies: {}
# [doc = " A tree-like collection of data [`Resource`]s primarily intended for storing"] # [doc = " locale and other internationalization data for [ICU] (International"] # [doc = " Components for Unicode)."] # [doc = ""] # [doc = " [ICU]: https://icu.unicode.org/"] # [derive (Debug)] pub struct ResourceBundle < 'a > { name : Cow < 'a , str > , root : Resource < 'a > , # [doc = " Whether fallback is enabled for this resource bundle."] # [doc = ""] # [doc = " A resource bundle storing locale data may omit some data in order to"] # [doc = " reduce duplication, allowing fallback to more general locales which"] # [doc = " use the same values."] pub is_locale_fallback_enabled : bool , }
};
}
