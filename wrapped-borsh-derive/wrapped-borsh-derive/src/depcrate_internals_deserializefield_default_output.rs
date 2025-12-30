// Generated macro for field_default_output (function)
macro_rules! Depcrate_internals_deserializefield_default_output {
() => {
// Module: crate::internals::deserialize
// Provides: {"field_default_output"}
// Dependencies: {}
# [doc = " function which computes derive output [proc_macro2::TokenStream]"] # [doc = " of code, which deserializes single skipped field"] fn field_default_output (field_name : Option < & Ident >) -> TokenStream2 { if let Some (field_name) = field_name { quote ! { # field_name : core :: default :: Default :: default () , } } else { quote ! { core :: default :: Default :: default () , } } }
};
}
