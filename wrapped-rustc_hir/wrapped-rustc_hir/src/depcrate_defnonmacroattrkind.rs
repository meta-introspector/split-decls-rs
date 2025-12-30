// Generated macro for NonMacroAttrKind (enum)
macro_rules! Depcrate_defNonMacroAttrKind {
() => {
// Module: crate::def
// Provides: {"NonMacroAttrKind"}
// Dependencies: {}
# [doc = " An attribute that is not a macro; e.g., `#[inline]` or `#[rustfmt::skip]`."] # [derive (Clone , Copy , PartialEq , Eq , Encodable , Decodable , Hash , Debug , HashStable_Generic)] pub enum NonMacroAttrKind { # [doc = " Single-segment attribute defined by the language (`#[inline]`)"] Builtin (Symbol) , # [doc = " Multi-segment custom attribute living in a \"tool module\" (`#[rustfmt::skip]`)."] Tool , # [doc = " Single-segment custom attribute registered by a derive macro (`#[serde(default)]`)."] DeriveHelper , # [doc = " Single-segment custom attribute registered by a derive macro"] # [doc = " but used before that derive macro was expanded (deprecated)."] DeriveHelperCompat , }
};
}
