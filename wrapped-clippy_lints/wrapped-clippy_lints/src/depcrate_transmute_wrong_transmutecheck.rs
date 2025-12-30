// Generated macro for check (function)
macro_rules! Depcrate_transmute_wrong_transmutecheck {
() => {
// Module: crate::transmute::wrong_transmute
// Provides: {"check"}
// Dependencies: {}
# [doc = " Checks for `wrong_transmute` lint."] # [doc = " Returns `true` if it's triggered, otherwise returns `false`."] pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ > , from_ty : Ty < 'tcx > , to_ty : Ty < 'tcx >) -> bool { match (& from_ty . kind () , & to_ty . kind ()) { (ty :: Float (_) | ty :: Char , ty :: Ref (..) | ty :: RawPtr (_ , _)) => { span_lint (cx , WRONG_TRANSMUTE , e . span , format ! ("transmute from a `{from_ty}` to a pointer") ,) ; true } , _ => false , } }
};
}
