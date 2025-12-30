// Generated macro for check_operand (function)
macro_rules! Depcrate_qualify_min_const_fncheck_operand {
() => {
// Module: crate::qualify_min_const_fn
// Provides: {"check_operand"}
// Dependencies: {}
fn check_operand < 'tcx > (cx : & LateContext < 'tcx > , operand : & Operand < 'tcx > , span : Span , body : & Body < 'tcx > , msrv : Msrv ,) -> McfResult { match operand { Operand :: Move (place) => { if ! place . projection . as_ref () . is_empty () && ! is_ty_const_destruct (cx . tcx , place . ty (& body . local_decls , cx . tcx) . ty , body) { return Err ((span , "cannot drop locals with a non constant destructor in const fn" . into () ,)) ; } check_place (cx , * place , span , body , msrv) } , Operand :: Copy (place) => check_place (cx , * place , span , body , msrv) , Operand :: Constant (c) => match c . check_static_ptr (cx . tcx) { Some (_) => Err ((span , "cannot access `static` items in const fn" . into ())) , None => Ok (()) , } , } }
};
}
