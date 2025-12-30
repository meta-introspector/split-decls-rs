// Generated macro for impl_208 (impl)
macro_rules! Depcrate_internals_serializeimpl_208 {
() => {
// Module: crate::internals::serialize
// Provides: {"impl_208"}
// Dependencies: {}
impl GenericsOutput { fn new (generics : & Generics) -> Self { Self { overrides : vec ! [] , serialize_visitor : generics :: FindTyParams :: new (generics) , } } fn extend (self , where_clause : & mut syn :: WhereClause , cratename : & Path) { let trait_path : Path = syn :: parse2 (quote ! { # cratename :: ser :: BorshSerialize }) . unwrap () ; let predicates = generics :: compute_predicates (self . serialize_visitor . process_for_bounds () , & trait_path) ; where_clause . predicates . extend (predicates) ; where_clause . predicates . extend (self . overrides) ; } }
};
}
