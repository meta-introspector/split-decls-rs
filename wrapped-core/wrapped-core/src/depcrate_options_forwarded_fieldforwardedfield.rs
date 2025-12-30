// Generated macro for ForwardedField (struct)
macro_rules! Depcrate_options_forwarded_fieldForwardedField {
() => {
// Module: crate::options::forwarded_field
// Provides: {"ForwardedField"}
// Dependencies: {}
# [doc = " A forwarded field and attributes that influence its behavior."] # [derive (Debug , Clone)] pub struct ForwardedField { # [doc = " The ident of the field that will receive the forwarded value."] pub ident : Ident , # [doc = " Path of the function that will be called to convert the forwarded value"] # [doc = " into the type expected by the field in `ident`."] pub with : Option < Path > , }
};
}
