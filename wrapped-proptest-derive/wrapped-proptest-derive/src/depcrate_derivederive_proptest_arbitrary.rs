// Generated macro for derive_proptest_arbitrary (function)
macro_rules! Depcrate_derivederive_proptest_arbitrary {
() => {
// Module: crate::derive
// Provides: {"derive_proptest_arbitrary"}
// Dependencies: {}
# [doc = " Entry point for deriving `Arbitrary`."] fn derive_proptest_arbitrary (ctx : Ctx , ast : DeriveInput ,) -> DeriveResult < TokenStream > { use syn :: Data :: * ; error :: if_has_lifetimes (ctx , & ast) ; let attrs = attr :: parse_top_attributes (ctx , & ast . attrs) ? ; let mut tracker = UseTracker :: new (ast . generics) ; if attrs . no_bound { tracker . no_track () ; } let the_impl = match ast . data { Struct (data) => derive_struct (ctx , DeriveData { tracker , attrs , ident : ast . ident , body : fields_to_vec (data . fields) , } ,) , Enum (data) => derive_enum (ctx , DeriveData { tracker , attrs , ident : ast . ident , body : data . variants . into_iter () . collect () , } ,) , _ => error :: not_struct_or_enum (ctx) ? , } ? ; let q = the_impl . into_tokens (ctx) ? ; Ok (q) }
};
}
