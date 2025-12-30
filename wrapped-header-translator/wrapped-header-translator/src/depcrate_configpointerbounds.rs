// Generated macro for PointerBounds (enum)
macro_rules! Depcrate_configPointerBounds {
() => {
// Module: crate::config
// Provides: {"PointerBounds"}
// Dependencies: {}
# [doc = " The bounds of a raw pointer."] # [doc = ""] # [doc = " Modelled after <https://clang.llvm.org/docs/BoundsSafety.html>."] # [derive (Deserialize , Debug , Clone , PartialEq , Eq , Hash , Default)] # [serde (deny_unknown_fields)] pub enum PointerBounds { # [default] # [serde (rename = "unspecified")] Unspecified , # [serde (rename = "unsafe")] Unsafe , # [serde (rename = "single")] Single , # [serde (rename = "null-terminated")] NullTerminated , # [serde (rename = "counted-by")] CountedBy (String) , # [serde (rename = "sized-by")] SizedBy (String) , # [serde (rename = "ended-by")] EndedBy (String) , }
};
}
