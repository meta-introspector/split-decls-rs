// Generated macro for impl_169 (impl)
macro_rules! Depcrate_internals_schemaimpl_169 {
() => {
// Module: crate::internals::schema
// Provides: {"impl_169"}
// Dependencies: {}
impl GenericsOutput { fn new (generics : & Generics) -> Self { Self { params_visitor : generics :: FindTyParams :: new (generics) , } } fn result (self , item_name : & str , cratename : & Path) -> (Vec < WherePredicate > , TokenStream2) { let trait_path : Path = syn :: parse2 (quote ! { # cratename :: BorshSchema }) . unwrap () ; let predicates = generics :: compute_predicates (self . params_visitor . clone () . process_for_bounds () , & trait_path ,) ; let declaration = declaration (item_name , cratename . clone () , self . params_visitor . process_for_bounds () ,) ; (predicates , declaration) } }
};
}
