// Generated macro for context_ty (function)
macro_rules! Depcrate_common_parse_downcastercontext_ty {
() => {
// Module: crate::common::parse::downcaster
// Provides: {"context_ty"}
// Dependencies: {}
# [doc = " Parses context type used for downcasting from the downcaster method signature."] # [doc = ""] # [doc = " Returns [`None`] if downcaster method doesn't accept context."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " If input arguments are invalid, then returns the [`Span`] to display the corresponding error at."] pub (crate) fn context_ty (sig : & syn :: Signature) -> Result < Option < syn :: Type > , Span > { match sig . receiver () { Some (rcv) => { if rcv . reference . is_none () || rcv . mutability . is_some () { return Err (rcv . span ()) ; } } _ => return Err (sig . span ()) , } if sig . inputs . len () > 2 { return Err (sig . inputs . span ()) ; } let second_arg_ty = match sig . inputs . iter () . nth (1) { Some (syn :: FnArg :: Typed (arg)) => & * arg . ty , None => return Ok (None) , _ => return Err (sig . inputs . span ()) , } ; match second_arg_ty . unparenthesized () { syn :: Type :: Reference (ref_ty) => { if ref_ty . mutability . is_some () { return Err (ref_ty . span ()) ; } Ok (Some (ref_ty . elem . unparenthesized () . clone ())) } ty => Err (ty . span ()) , } }
};
}
