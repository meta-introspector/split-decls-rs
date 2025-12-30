// Generated macro for FieldLevelSetter (struct)
macro_rules! Depcrate_macro_optionsFieldLevelSetter {
() => {
// Module: crate::macro_options
// Provides: {"FieldLevelSetter"}
// Dependencies: {}
# [doc = " The `setter` meta item on fields in the input type."] # [doc = " Unlike the `setter` meta item at the struct level, this allows specific"] # [doc = " name overrides."] # [derive (Debug , Clone , Default , FromMeta)] pub struct FieldLevelSetter { # [doc = " A prefix that will be added to the field ident to name the setter."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust,ignore"] # [doc = " #[derive(Builder)]"] # [doc = " struct Example {"] # [doc = "     #[builder(setter(prefix = \"with\"))]"] # [doc = "     name: String,"] # [doc = " }"] # [doc = ""] # [doc = " let example = ExampleBuilder::default()"] # [doc = "     .set_name(\"John\".to_string())"] # [doc = "     .build()?;"] # [doc = " ```"] prefix : Option < Ident > , # [doc = " A custom name for the setter method. This overrules the `prefix` field."] name : Option < Ident > , # [doc = " If `true`, this setter takes an argument that impls `Into<T>`, where `T` is the type of that field"] # [doc = " in the deriving struct. Otherwise, the argument's type will be `T`."] into : Option < bool > , strip_option : Option < bool > , skip : Option < bool > , custom : Option < bool > , # [darling (with = parse_each)] each : Option < Each > , }
};
}
