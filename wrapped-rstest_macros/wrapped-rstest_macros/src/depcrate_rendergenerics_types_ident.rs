// Generated macro for generics_types_ident (function)
macro_rules! Depcrate_rendergenerics_types_ident {
() => {
// Module: crate::render
// Provides: {"generics_types_ident"}
// Dependencies: {}
fn generics_types_ident (generics : & syn :: Generics) -> impl Iterator < Item = & '_ Ident > { generics . type_params () . map (| tp | & tp . ident) }
};
}
