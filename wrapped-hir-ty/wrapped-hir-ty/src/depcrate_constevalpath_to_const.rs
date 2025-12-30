// Generated macro for path_to_const (function)
macro_rules! Depcrate_constevalpath_to_const {
() => {
// Module: crate::consteval
// Provides: {"path_to_const"}
// Dependencies: {}
pub (crate) fn path_to_const < 'g > (db : & dyn HirDatabase , resolver : & Resolver < '_ > , path : & Path , mode : ParamLoweringMode , args : impl FnOnce () -> & 'g Generics , debruijn : DebruijnIndex , expected_ty : Ty ,) -> Option < Const > { match resolver . resolve_path_in_value_ns_fully (db , path , HygieneId :: ROOT) { Some (ValueNs :: GenericParam (p)) => { let ty = db . const_param_ty (p) ; let value = match mode { ParamLoweringMode :: Placeholder => { ConstValue :: Placeholder (to_placeholder_idx (db , p . into ())) } ParamLoweringMode :: Variable => { let args = args () ; match args . type_or_const_param_idx (p . into ()) { Some (it) => ConstValue :: BoundVar (BoundVar :: new (debruijn , it)) , None => { never ! ("Generic list doesn't contain this param: {:?}, {:?}, {:?}" , args , path , p) ; return None ; } } } } ; Some (ConstData { ty , value } . intern (Interner)) } Some (ValueNs :: ConstId (c)) => Some (intern_const_scalar (ConstScalar :: UnevaluatedConst (c . into () , Substitution :: empty (Interner)) , expected_ty ,)) , _ => None , } }
};
}
