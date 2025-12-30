// Generated macro for impl_399 (impl)
macro_rules! Depcrate_scriptimpl_399 {
() => {
// Module: crate::script
// Provides: {"impl_399"}
// Dependencies: {}
impl < 'a > ScriptExtensionsSet < 'a > { # [doc = " Returns whether this set contains the given script."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::props::Script;"] # [doc = " use icu::properties::script::ScriptWithExtensions;"] # [doc = " let swe = ScriptWithExtensions::new();"] # [doc = ""] # [doc = " assert!(swe"] # [doc = "     .get_script_extensions_val('\\u{11303}') // GRANTHA SIGN VISARGA"] # [doc = "     .contains(&Script::Grantha));"] # [doc = " ```"] pub fn contains (& self , x : & Script) -> bool { ZeroSlice :: binary_search (self . values , x) . is_ok () } # [doc = " Gets an iterator over the elements."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::props::Script;"] # [doc = " use icu::properties::script::ScriptWithExtensions;"] # [doc = " let swe = ScriptWithExtensions::new();"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     swe.get_script_extensions_val('௫') // U+0BEB TAMIL DIGIT FIVE"] # [doc = "         .iter()"] # [doc = "         .collect::<Vec<_>>(),"] # [doc = "     [Script::Tamil, Script::Grantha]"] # [doc = " );"] # [doc = " ```"] pub fn iter (& self) -> impl DoubleEndedIterator < Item = Script > + 'a { ZeroSlice :: iter (self . values) } # [doc = " For accessing this set as an array instead of an iterator"] # [doc (hidden)] pub fn array_len (& self) -> usize { self . values . len () } # [doc = " For accessing this set as an array instead of an iterator"] # [doc (hidden)] pub fn array_get (& self , index : usize) -> Option < Script > { self . values . get (index) } }
};
}
