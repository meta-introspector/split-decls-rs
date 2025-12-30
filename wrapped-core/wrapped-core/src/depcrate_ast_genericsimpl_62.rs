// Generated macro for impl_62 (impl)
macro_rules! Depcrate_ast_genericsimpl_62 {
() => {
// Module: crate::ast::generics
// Provides: {"impl_62"}
// Dependencies: {}
impl < P : FromGenericParam > FromGenerics for Generics < P > { fn from_generics (generics : & syn :: Generics) -> Result < Self > { Ok (Generics { params : generics . params . iter () . map (FromGenericParam :: from_generic_param) . collect :: < Result < Vec < P > > > () ? , where_clause : generics . where_clause . clone () , }) } }
};
}
