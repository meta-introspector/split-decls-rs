// Generated macro for generate (function)
macro_rules! Depcrate_visit_mutgenerate {
() => {
// Module: crate::visit_mut
// Provides: {"generate"}
// Dependencies: {}
pub fn generate (defs : & Definitions) -> Result < () > { let (traits , impls) = gen :: traverse (defs , node) ; let full_macro = full :: get_macro () ; file :: write (VISIT_MUT_SRC , quote ! { #! [allow (unused_variables)] #! [allow (clippy :: needless_pass_by_ref_mut)] # [cfg (any (feature = "full" , feature = "derive"))] use crate :: punctuated :: Punctuated ; # full_macro macro_rules ! skip { ($ ($ tt : tt) *) => { } ; } # [doc = " Syntax tree traversal to mutate an exclusive borrow of a syntax tree in"] # [doc = " place."] # [doc = ""] # [doc = " See the [module documentation] for details."] # [doc = ""] # [doc = " [module documentation]: self"] pub trait VisitMut { # traits } # impls } ,) ? ; Ok (()) }
};
}
