// Generated macro for CanonicalClassQuery (enum)
macro_rules! Depcrate_unicodeCanonicalClassQuery {
() => {
// Module: crate::unicode
// Provides: {"CanonicalClassQuery"}
// Dependencies: {}
# [doc = " Like ClassQuery, but its parameters have been canonicalized. This also"] # [doc = " differentiates binary properties from flattened general categories and"] # [doc = " scripts."] # [derive (Debug , Eq , PartialEq)] enum CanonicalClassQuery { # [doc = " The canonical binary property name."] Binary (& 'static str) , # [doc = " The canonical general category name."] GeneralCategory (& 'static str) , # [doc = " The canonical script name."] Script (& 'static str) , # [doc = " An arbitrary association between property and value, both of which"] # [doc = " have been canonicalized."] # [doc = ""] # [doc = " Note that by construction, the property name of ByValue will never"] # [doc = " be General_Category or Script. Those two cases are subsumed by the"] # [doc = " eponymous variants."] ByValue { # [doc = " The canonical property name."] property_name : & 'static str , # [doc = " The canonical property value."] property_value : & 'static str , } , }
};
}
