// Generated macro for check (function)
macro_rules! Depcrate_transmute_crosspointer_transmutecheck {
() => {
// Module: crate::transmute::crosspointer_transmute
// Provides: {"check"}
// Dependencies: {}
# [doc = " Checks for `crosspointer_transmute` lint."] # [doc = " Returns `true` if it's triggered, otherwise returns `false`."] pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ > , from_ty : Ty < 'tcx > , to_ty : Ty < 'tcx >) -> bool { match (* from_ty . kind () , * to_ty . kind ()) { (ty :: RawPtr (from_ptr_ty , _) , _) if from_ptr_ty == to_ty => { span_lint (cx , CROSSPOINTER_TRANSMUTE , e . span , format ! ("transmute from a type (`{from_ty}`) to the type that it points to (`{to_ty}`)") ,) ; true } , (_ , ty :: RawPtr (to_ptr_ty , _)) if to_ptr_ty == from_ty => { span_lint (cx , CROSSPOINTER_TRANSMUTE , e . span , format ! ("transmute from a type (`{from_ty}`) to a pointer to that type (`{to_ty}`)") ,) ; true } , _ => false , } }
};
}
