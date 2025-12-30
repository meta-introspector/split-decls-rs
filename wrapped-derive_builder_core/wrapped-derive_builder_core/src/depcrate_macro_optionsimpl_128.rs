// Generated macro for impl_128 (impl)
macro_rules! Depcrate_macro_optionsimpl_128 {
() => {
// Module: crate::macro_options
// Provides: {"impl_128"}
// Dependencies: {}
# [doc = " Converters to codegen structs"] impl < 'a > FieldWithDefaults < 'a > { # [doc = " Returns a `Setter` according to the options."] pub fn as_setter (& 'a self) -> Setter < 'a > { Setter { crate_root : & self . parent . crate_root , setter_enabled : self . setter_enabled () , try_setter : self . try_setter () , visibility : self . setter_vis () , pattern : self . pattern () , attrs : & self . field . attrs . setter , ident : self . setter_ident () , field_ident : self . field_ident () , field_type : self . field_type () , generic_into : self . setter_into () , strip_option : self . setter_strip_option () , each : self . field . setter . each . as_ref () , } } # [doc = " Returns an `Initializer` according to the options."] pub fn as_initializer (& 'a self) -> Initializer < 'a > { Initializer { crate_root : & self . parent . crate_root , field_enabled : self . field_enabled () , field_ident : self . field_ident () , builder_pattern : self . pattern () , default_value : self . field . default . as_ref () , use_default_struct : self . use_parent_default () , conversion : self . conversion () , custom_error_type_span : self . parent . build_fn . error . as_ref () . and_then (| err_ty | { match err_ty { BuildFnError :: Existing (p) => Some (p . span ()) , _ => None , } }) , } } pub fn as_builder_field (& 'a self) -> BuilderField < 'a > { BuilderField { crate_root : & self . parent . crate_root , field_ident : self . field_ident () , field_type : self . field_type () , field_visibility : self . field_vis () , attrs : & self . field . attrs . field , } } }
};
}
