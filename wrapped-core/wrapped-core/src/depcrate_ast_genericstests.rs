// Generated macro for tests (module)
macro_rules! Depcrate_ast_genericstests {
() => {
// Module: crate::ast::generics
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use syn :: parse_quote ; use super :: { GenericParam , Generics } ; use crate :: FromGenerics ; # [test] fn generics () { let g : syn :: Generics = parse_quote ! (< T >) ; let deified : Generics < GenericParam < syn :: Ident > > = FromGenerics :: from_generics (& g) . unwrap () ; assert ! (deified . params . len () == 1) ; assert ! (deified . where_clause . is_none ()) ; } }
};
}
