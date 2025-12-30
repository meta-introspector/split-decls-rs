// Generated macro for property_functions (macro)
macro_rules! Depcrate_unicodeproperty_functions {
() => {
// Module: crate::unicode
// Provides: {"property_functions"}
// Dependencies: {}
macro_rules ! property_functions { ($ module : ident , $ property_names : ident , [$ ($ prop : ident ,) *]) => { # [allow (unused)] mod $ module ; $ (pub fn $ prop (c : char) -> bool { self ::$ module ::$ prop . contains_char (c) }) * pub static $ property_names : & [& str] = & [$ (stringify ! ($ prop) ,) *] ; } ; }
};
}
