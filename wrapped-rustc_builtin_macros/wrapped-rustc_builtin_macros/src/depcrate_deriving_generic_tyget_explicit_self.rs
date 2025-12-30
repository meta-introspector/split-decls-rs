// Generated macro for get_explicit_self (function)
macro_rules! Depcrate_deriving_generic_tyget_explicit_self {
() => {
// Module: crate::deriving::generic::ty
// Provides: {"get_explicit_self"}
// Dependencies: {}
pub (crate) fn get_explicit_self (cx : & ExtCtxt < '_ > , span : Span) -> (Box < Expr > , ast :: ExplicitSelf) { let self_path = cx . expr_self (span) ; let self_ty = respan (span , SelfKind :: Region (None , ast :: Mutability :: Not)) ; (self_path , self_ty) }
};
}
