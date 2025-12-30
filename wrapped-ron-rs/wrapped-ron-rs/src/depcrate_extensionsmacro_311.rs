// Generated macro for macro_311 (macro)
macro_rules! Depcrate_extensionsmacro_311 {
() => {
// Module: crate::extensions
// Provides: {"macro_311"}
// Dependencies: {}
bitflags :: bitflags ! { # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash , Serialize , Deserialize)] pub struct Extensions : usize { const UNWRAP_NEWTYPES = 0x1 ; const IMPLICIT_SOME = 0x2 ; const UNWRAP_VARIANT_NEWTYPES = 0x4 ; # [doc = " During serialization, this extension emits struct names. See also [`PrettyConfig::struct_names`](crate::ser::PrettyConfig::struct_names) for the [`PrettyConfig`](crate::ser::PrettyConfig) equivalent."] # [doc = ""] # [doc = " During deserialization, this extension requires that structs' names are stated explicitly."] const EXPLICIT_STRUCT_NAMES = 0x8 ; } }
};
}
