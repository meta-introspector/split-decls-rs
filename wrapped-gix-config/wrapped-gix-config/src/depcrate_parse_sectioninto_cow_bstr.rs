// Generated macro for into_cow_bstr (function)
macro_rules! Depcrate_parse_sectioninto_cow_bstr {
() => {
// Module: crate::parse::section
// Provides: {"into_cow_bstr"}
// Dependencies: {}
pub (crate) fn into_cow_bstr (c : Cow < '_ , str >) -> Cow < '_ , BStr > { match c { Cow :: Borrowed (s) => Cow :: Borrowed (s . into ()) , Cow :: Owned (s) => Cow :: Owned (s . into ()) , } }
};
}
