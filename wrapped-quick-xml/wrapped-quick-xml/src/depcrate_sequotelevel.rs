// Generated macro for QuoteLevel (enum)
macro_rules! Depcrate_seQuoteLevel {
() => {
// Module: crate::se
// Provides: {"QuoteLevel"}
// Dependencies: {}
# [doc = " Defines which characters would be escaped in [`Text`] events and attribute"] # [doc = " values."] # [doc = ""] # [doc = " [`Text`]: crate::events::Event::Text"] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum QuoteLevel { # [doc = " Performs escaping, escape all characters that could have special meaning"] # [doc = " in the XML. This mode is compatible with SGML specification."] # [doc = ""] # [doc = " Characters that will be replaced:"] # [doc = ""] # [doc = " Original | Replacement"] # [doc = " ---------|------------"] # [doc = " `<`      | `&lt;`"] # [doc = " `>`      | `&gt;`"] # [doc = " `&`      | `&amp;`"] # [doc = " `\"`      | `&quot;`"] # [doc = " `'`      | `&apos;`"] Full , # [doc = " Performs escaping that is compatible with SGML specification."] # [doc = ""] # [doc = " This level adds escaping of `>` to the `Minimal` level, which is [required]"] # [doc = " for compatibility with SGML."] # [doc = ""] # [doc = " Characters that will be replaced:"] # [doc = ""] # [doc = " Original | Replacement"] # [doc = " ---------|------------"] # [doc = " `<`      | `&lt;`"] # [doc = " `>`      | `&gt;`"] # [doc = " `&`      | `&amp;`"] # [doc = ""] # [doc = " [required]: https://www.w3.org/TR/xml11/#syntax"] Partial , # [doc = " Performs the minimal possible escaping, escape only strictly necessary"] # [doc = " characters."] # [doc = ""] # [doc = " Characters that will be replaced:"] # [doc = ""] # [doc = " Original | Replacement"] # [doc = " ---------|------------"] # [doc = " `<`      | `&lt;`"] # [doc = " `&`      | `&amp;`"] Minimal , }
};
}
