// Generated macro for expand_deriving_unsized_const_param_ty (function)
macro_rules! Depcrate_deriving_boundsexpand_deriving_unsized_const_param_ty {
() => {
// Module: crate::deriving::bounds
// Provides: {"expand_deriving_unsized_const_param_ty"}
// Dependencies: {}
pub (crate) fn expand_deriving_unsized_const_param_ty (cx : & ExtCtxt < '_ > , span : Span , mitem : & MetaItem , item : & Annotatable , push : & mut dyn FnMut (Annotatable) , is_const : bool ,) { let trait_def = TraitDef { span , path : path_std ! (marker :: UnsizedConstParamTy) , skip_path_as_bound : false , needs_copy_as_bound_if_packed : false , additional_bounds : vec ! [ty :: Ty :: Path (path_std ! (cmp :: Eq))] , supports_unions : false , methods : Vec :: new () , associated_types : Vec :: new () , is_const , is_staged_api_crate : cx . ecfg . features . staged_api () , } ; trait_def . expand (cx , mitem , item , push) ; }
};
}
