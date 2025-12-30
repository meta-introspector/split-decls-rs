// Generated macro for Script (struct)
macro_rules! Depcrate_propsScript {
() => {
// Module: crate::props
// Provides: {"Script"}
// Dependencies: {}
# [doc = " Enumerated property Script."] # [doc = ""] # [doc = " This is used with both the Script and Script_Extensions Unicode properties."] # [doc = " Each character is assigned a single Script, but characters that are used in"] # [doc = " a particular subset of scripts will be in more than one Script_Extensions set."] # [doc = " For example, DEVANAGARI DIGIT NINE has Script=Devanagari, but is also in the"] # [doc = " Script_Extensions set for Dogra, Kaithi, and Mahajani. If you are trying to"] # [doc = " determine whether a code point belongs to a certain script, you should use"] # [doc = " [`ScriptWithExtensionsBorrowed::has_script`]."] # [doc = ""] # [doc = " For more information, see UAX #24: <https://www.unicode.org/reports/tr24/>."] # [doc = " See `UScriptCode` in ICU4C."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::{CodePointMapData, props::Script};"] # [doc = ""] # [doc = " assert_eq!(CodePointMapData::<Script>::new().get('木'), Script::Han);  // U+6728"] # [doc = " assert_eq!(CodePointMapData::<Script>::new().get('🎃'), Script::Common);  // U+1F383 JACK-O-LANTERN"] # [doc = " ```"] # [doc = " [`ScriptWithExtensionsBorrowed::has_script`]: crate::script::ScriptWithExtensionsBorrowed::has_script"] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [allow (clippy :: exhaustive_structs)] # [repr (transparent)] pub struct Script (pub (crate) u16) ;
};
}
