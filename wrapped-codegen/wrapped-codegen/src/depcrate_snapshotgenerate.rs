// Generated macro for generate (function)
macro_rules! Depcrate_snapshotgenerate {
() => {
// Module: crate::snapshot
// Provides: {"generate"}
// Dependencies: {}
pub fn generate (defs : & Definitions) -> Result < () > { let mut impls = TokenStream :: new () ; for node in & defs . types { impls . extend (expand_impl (defs , node)) ; } for (name , symbol) in & defs . tokens { impls . extend (expand_token_impl (name , symbol)) ; } file :: write (TESTS_DEBUG_SRC , quote ! { #! [allow (repr_transparent_non_zst_fields)] #! [allow (clippy :: match_wildcard_for_single_variants)] use super :: { Lite , Present } ; use ref_cast :: RefCast ; use std :: fmt :: { self , Debug , Display } ; # impls } ,) ? ; Ok (()) }
};
}
