// Generated macro for generate (function)
macro_rules! Depcrate_clonegenerate {
() => {
// Module: crate::clone
// Provides: {"generate"}
// Dependencies: {}
pub fn generate (defs : & Definitions) -> Result < () > { let mut impls = TokenStream :: new () ; for node in & defs . types { impls . extend (expand_impl (defs , node)) ; } file :: write (CLONE_SRC , quote ! { #! [allow (clippy :: clone_on_copy , clippy :: expl_impl_clone_on_copy)] # impls } ,) ? ; Ok (()) }
};
}
