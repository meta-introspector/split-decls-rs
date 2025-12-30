// Generated macro for generate (function)
macro_rules! Depcrate_debuggenerate {
() => {
// Module: crate::debug
// Provides: {"generate"}
// Dependencies: {}
pub fn generate (defs : & Definitions) -> Result < () > { let mut syntax_tree_variants = Set :: new () ; for node in & defs . types { if let Data :: Enum (variants) = & node . data { let enum_name = & node . ident ; for (variant_name , fields) in variants { if let Some (inner) = syntax_tree_enum (enum_name , variant_name , fields) { syntax_tree_variants . insert (inner) ; } } } } let mut impls = TokenStream :: new () ; for node in & defs . types { impls . extend (expand_impl (defs , node , & syntax_tree_variants)) ; } file :: write (DEBUG_SRC , quote ! { #! [allow (unknown_lints , non_local_definitions)] use std :: fmt :: { self , Debug } ; # impls } ,) ? ; Ok (()) }
};
}
