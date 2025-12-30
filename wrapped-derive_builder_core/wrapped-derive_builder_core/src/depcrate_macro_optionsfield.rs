// Generated macro for Field (struct)
macro_rules! Depcrate_macro_optionsField {
() => {
// Module: crate::macro_options
// Provides: {"Field"}
// Dependencies: {}
# [doc = " Data extracted from the fields of the input struct."] # [derive (Debug , Clone , FromField)] # [darling (attributes (builder) , forward_attrs (doc , cfg , allow , builder_field_attr , builder_setter_attr) , and_then = "Self::resolve")] pub struct Field { ident : Option < Ident > , # [darling (with = TryFrom :: try_from)] attrs : FieldForwardedAttrs , ty : syn :: Type , # [doc = " Field-level override for builder pattern."] # [doc = " Note that setting this may force the builder to derive `Clone`."] pattern : Option < BuilderPattern > , # [darling (flatten)] visibility : VisibilityAttr , # [darling (default , with = field_setter)] setter : FieldLevelSetter , # [doc = " The value for this field if the setter is never invoked."] # [doc = ""] # [doc = " A field can get its default one of three ways:"] # [doc = ""] # [doc = " 1. An explicit `default = \"...\"` expression"] # [doc = " 2. An explicit `default` word, in which case the field type's `Default::default()`"] # [doc = "    value is used"] # [doc = " 3. Inherited from the field's value in the struct's `default` value."] # [doc = ""] # [doc = " This property only captures the first two, the third is computed in `FieldWithDefaults`."] default : Option < DefaultExpression > , try_setter : Flag , # [darling (default)] field : FieldLevelFieldMeta , }
};
}
