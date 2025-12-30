// Generated macro for load_aliases (function)
macro_rules! Depcrate_parseload_aliases {
() => {
// Module: crate::parse
// Provides: {"load_aliases"}
// Dependencies: {}
fn load_aliases (use_tree : UseTree , lookup : & mut Lookup) { match use_tree { UseTree :: Path (use_tree) => load_aliases (* use_tree . tree , lookup) , UseTree :: Rename (use_tree) => { lookup . aliases . insert (use_tree . rename , use_tree . ident) ; } UseTree :: Group (use_tree) => { for use_tree in use_tree . items { load_aliases (use_tree , lookup) ; } } UseTree :: Name (_) | UseTree :: Glob (_) => { } } }
};
}
