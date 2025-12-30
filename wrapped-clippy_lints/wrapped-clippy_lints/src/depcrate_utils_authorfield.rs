// Generated macro for field (macro)
macro_rules! Depcrate_utils_authorfield {
() => {
// Module: crate::utils::author
// Provides: {"field"}
// Dependencies: {}
# [doc = " Creates a `Binding` that accesses the field of an existing `Binding`"] macro_rules ! field { ($ binding : ident .$ field : ident) => { & Binding { name : $ binding . name . to_string () + stringify ! (.$ field) , value : $ binding . value .$ field , } } ; }
};
}
