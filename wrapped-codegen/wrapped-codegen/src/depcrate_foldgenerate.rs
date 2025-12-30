// Generated macro for generate (function)
macro_rules! Depcrate_foldgenerate {
() => {
// Module: crate::fold
// Provides: {"generate"}
// Dependencies: {}
pub fn generate (defs : & Definitions) -> Result < () > { let (traits , impls) = gen :: traverse (defs , node) ; let full_macro = full :: get_macro () ; file :: write (FOLD_SRC , quote ! { #! [allow (unreachable_code , unused_variables)] #! [allow (clippy :: match_wildcard_for_single_variants , clippy :: needless_match , clippy :: needless_pass_by_ref_mut ,)] # full_macro # [doc = " Syntax tree traversal to transform the nodes of an owned syntax tree."] # [doc = ""] # [doc = " See the [module documentation] for details."] # [doc = ""] # [doc = " [module documentation]: self"] pub trait Fold { # traits } # impls # [cfg (any (feature = "derive" , feature = "full"))] fn fold_vec < T , V , F > (vec : Vec < T >, fold : & mut V , mut f : F) -> Vec < T > where V : ? Sized , F : FnMut (& mut V , T) -> T , { vec . into_iter () . map (| it | f (fold , it)) . collect () } } ,) ? ; Ok (()) }
};
}
