// Generated macro for trait_visibility (function)
macro_rules! Depcrate_visibilitytrait_visibility {
() => {
// Module: crate::visibility
// Provides: {"trait_visibility"}
// Dependencies: {}
fn trait_visibility (db : & dyn DefDatabase , def : TraitId) -> Visibility { let loc = def . lookup (db) ; let source = loc . source (db) ; visibility_from_ast (db , def , source . map (| src | src . visibility ())) }
};
}
