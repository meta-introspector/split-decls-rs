// Generated macro for StructLevelSetter (struct)
macro_rules! Depcrate_macro_optionsStructLevelSetter {
() => {
// Module: crate::macro_options
// Provides: {"StructLevelSetter"}
// Dependencies: {}
# [derive (Debug , Clone , Default , FromMeta)] pub struct StructLevelSetter { # [doc = " A prefix that will be added to the idents for generated setters."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust,ignore"] # [doc = " #[derive(Builder)]"] # [doc = " #[builder(setter(prefix = \"with\"))]"] # [doc = " struct Example {"] # [doc = "     name: String,"] # [doc = " }"] # [doc = ""] # [doc = " let example = ExampleBuilder::default()"] # [doc = "     .set_name(\"John\".to_string())"] # [doc = "     .build()?;"] # [doc = " ```"] prefix : Option < Ident > , # [doc = " If `true`, setters will default to taking an argument that impls `Into<T>`, where `T` is the type of that field"] # [doc = " in the deriving struct."] into : Option < bool > , # [doc = " If `true`, setters will default to stripping `Option` type when it's encountered on fields in the deriving struct."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust,ignore"] # [doc = " #[derive(Builder)]"] # [doc = " struct Example {"] # [doc = "     title: Option<&'static str>,"] # [doc = "     name: &'static str,"] # [doc = " }"] # [doc = ""] # [doc = ""] # [doc = " ```"] strip_option : Option < bool > , # [doc = " If `true`, setters will only be generated for fields that opt-in."] skip : Option < bool > , }
};
}
