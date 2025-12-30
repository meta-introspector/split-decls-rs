// Generated macro for FieldLevelFieldMeta (struct)
macro_rules! Depcrate_macro_optionsFieldLevelFieldMeta {
() => {
// Module: crate::macro_options
// Provides: {"FieldLevelFieldMeta"}
// Dependencies: {}
# [doc = " Contents of the `field` meta in `builder` attributes at the field level."] # [derive (Debug , Clone , Default , FromMeta)] pub struct FieldLevelFieldMeta { # [darling (flatten)] visibility : VisibilityAttr , # [doc = " Custom builder field type"] # [darling (rename = "ty")] builder_type : Option < syn :: Type > , # [doc = " Custom builder field method, for making target struct field value"] build : Option < BlockContents > , }
};
}
