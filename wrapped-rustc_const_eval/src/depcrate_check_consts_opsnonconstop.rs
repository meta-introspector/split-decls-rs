// Generated macro for NonConstOp (trait)
macro_rules! Depcrate_check_consts_opsNonConstOp {
() => {
// Module: crate::check_consts::ops
// Provides: {"NonConstOp"}
// Dependencies: {}
# [doc = " An operation that is *not allowed* in a const context."] pub trait NonConstOp < 'tcx > : std :: fmt :: Debug { # [doc = " Returns an enum indicating whether this operation can be enabled with a feature gate."] fn status_in_item (& self , _ccx : & ConstCx < '_ , 'tcx >) -> Status { Status :: Forbidden } fn importance (& self) -> DiagImportance { DiagImportance :: Primary } fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > ; }
};
}
