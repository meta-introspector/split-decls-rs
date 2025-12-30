// Generated macro for impl_112 (impl)
macro_rules! Depcrate_internals_deserializeimpl_112 {
() => {
// Module: crate::internals::deserialize
// Provides: {"impl_112"}
// Dependencies: {}
impl GenericsOutput { fn new (generics : & Generics) -> Self { Self { overrides : vec ! [] , deserialize_visitor : generics :: FindTyParams :: new (generics) , default_visitor : generics :: FindTyParams :: new (generics) , } } fn extend (self , where_clause : & mut syn :: WhereClause , cratename : & Path) { let de_trait : Path = syn :: parse2 (quote ! { # cratename :: de :: BorshDeserialize }) . unwrap () ; let default_trait : Path = syn :: parse2 (quote ! { core :: default :: Default }) . unwrap () ; let de_predicates = generics :: compute_predicates (self . deserialize_visitor . process_for_bounds () , & de_trait) ; let default_predicates = generics :: compute_predicates (self . default_visitor . process_for_bounds () , & default_trait) ; where_clause . predicates . extend (de_predicates) ; where_clause . predicates . extend (default_predicates) ; where_clause . predicates . extend (self . overrides) ; } }
};
}
