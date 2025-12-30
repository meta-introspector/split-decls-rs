// Generated macro for Options (struct)
macro_rules! Depcrate_optionsOptions {
() => {
// Module: crate::options
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Roundtrip serde options."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use ron::{Options, extensions::Extensions};"] # [doc = ""] # [doc = " let ron = Options::default()"] # [doc = "     .with_default_extension(Extensions::IMPLICIT_SOME);"] # [doc = ""] # [doc = " let de: Option<i32> = ron.from_str(\"42\").unwrap();"] # [doc = " let ser = ron.to_string(&de).unwrap();"] # [doc = ""] # [doc = " assert_eq!(ser, \"42\");"] # [doc = " ```"] # [derive (Clone , Debug , Serialize , Deserialize)] # [serde (default)] # [non_exhaustive] pub struct Options { # [doc = " Extensions that are enabled by default during serialization and"] # [doc = "  deserialization."] # [doc = " During serialization, these extensions do NOT have to be explicitly"] # [doc = "  enabled in the parsed RON."] # [doc = " During deserialization, these extensions are used, but their explicit"] # [doc = "  activation is NOT included in the output RON."] # [doc = " No extensions are enabled by default."] pub default_extensions : Extensions , # [doc = " Default recursion limit that is checked during serialization and"] # [doc = "  deserialization."] # [doc = " If set to `None`, infinite recursion is allowed and stack overflow"] # [doc = "  errors can crash the serialization or deserialization process."] # [doc = " Defaults to `Some(128)`, i.e. 128 recursive calls are allowed."] pub recursion_limit : Option < usize > , }
};
}
