// Generated macro for FromGenericParam (trait)
macro_rules! Depcrate_from_generic_paramFromGenericParam {
() => {
// Module: crate::from_generic_param
// Provides: {"FromGenericParam"}
// Dependencies: {}
# [doc = " Creates an instance by parsing a specific `syn::GenericParam`."] # [doc = " This can be a type param, a lifetime, or a const param."] pub trait FromGenericParam : Sized { fn from_generic_param (param : & syn :: GenericParam) -> Result < Self > ; }
};
}
