// Generated macro for eq_ty (function)
macro_rules! Depcrate_ast_utilseq_ty {
() => {
// Module: crate::ast_utils
// Provides: {"eq_ty"}
// Dependencies: {}
pub fn eq_ty (l : & Ty , r : & Ty) -> bool { use TyKind :: * ; match (& l . kind , & r . kind) { (Paren (l) , _) => eq_ty (l , r) , (_ , Paren (r)) => eq_ty (l , r) , (Never , Never) | (Infer , Infer) | (ImplicitSelf , ImplicitSelf) | (Err (_) , Err (_)) | (CVarArgs , CVarArgs) => { true } , (Slice (l) , Slice (r)) => eq_ty (l , r) , (Array (le , ls) , Array (re , rs)) => eq_ty (le , re) && eq_expr (& ls . value , & rs . value) , (Ptr (l) , Ptr (r)) => l . mutbl == r . mutbl && eq_ty (& l . ty , & r . ty) , (Ref (ll , l) , Ref (rl , r)) => { both (ll . as_ref () , rl . as_ref () , | l , r | eq_id (l . ident , r . ident)) && l . mutbl == r . mutbl && eq_ty (& l . ty , & r . ty) } , (PinnedRef (ll , l) , PinnedRef (rl , r)) => { both (ll . as_ref () , rl . as_ref () , | l , r | eq_id (l . ident , r . ident)) && l . mutbl == r . mutbl && eq_ty (& l . ty , & r . ty) } , (FnPtr (l) , FnPtr (r)) => { l . safety == r . safety && eq_ext (& l . ext , & r . ext) && over (& l . generic_params , & r . generic_params , eq_generic_param) && eq_fn_decl (& l . decl , & r . decl) } , (Tup (l) , Tup (r)) => over (l , r , | l , r | eq_ty (l , r)) , (Path (lq , lp) , Path (rq , rp)) => both (lq . as_deref () , rq . as_deref () , eq_qself) && eq_path (lp , rp) , (TraitObject (lg , ls) , TraitObject (rg , rs)) => ls == rs && over (lg , rg , eq_generic_bound) , (ImplTrait (_ , lg) , ImplTrait (_ , rg)) => over (lg , rg , eq_generic_bound) , (Typeof (l) , Typeof (r)) => eq_expr (& l . value , & r . value) , (MacCall (l) , MacCall (r)) => eq_mac_call (l , r) , _ => false , } }
};
}
