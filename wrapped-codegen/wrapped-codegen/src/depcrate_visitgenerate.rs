// Generated macro for generate (function)
macro_rules! Depcrate_visitgenerate {
() => {
// Module: crate::visit
// Provides: {"generate"}
// Dependencies: {}
pub fn generate (defs : & Definitions) -> Result < () > { let (traits , impls) = gen :: traverse (defs , node) ; let full_macro = full :: get_macro () ; file :: write (VISIT_SRC , quote ! { #! [allow (unused_variables)] #! [allow (clippy :: needless_pass_by_ref_mut)] # [cfg (any (feature = "full" , feature = "derive"))] use crate :: punctuated :: Punctuated ; # full_macro macro_rules ! skip { ($ ($ tt : tt) *) => { } ; } # [doc = " Syntax tree traversal to walk a shared borrow of a syntax tree."] # [doc = ""] # [doc = " See the [module documentation] for details."] # [doc = ""] # [doc = " [module documentation]: self"] pub trait Visit <'ast > { # traits } # impls } ,) ? ; Ok (()) }
};
}
