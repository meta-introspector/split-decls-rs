// Generated macro for generate (function)
macro_rules! Depcrate_eqgenerate {
() => {
// Module: crate::eq
// Provides: {"generate"}
// Dependencies: {}
pub fn generate (defs : & Definitions) -> Result < () > { let mut impls = TokenStream :: new () ; for node in & defs . types { impls . extend (expand_impl (defs , node)) ; } file :: write (EQ_SRC , quote ! { # [cfg (any (feature = "derive" , feature = "full"))] use crate :: tt :: TokenStreamHelper ; # impls } ,) ? ; Ok (()) }
};
}
