// Generated macro for equate_intrinsic_type (function)
macro_rules! Depcrate_check_intrinsicequate_intrinsic_type {
() => {
// Module: crate::check::intrinsic
// Provides: {"equate_intrinsic_type"}
// Dependencies: {}
fn equate_intrinsic_type < 'tcx > (tcx : TyCtxt < 'tcx > , span : Span , def_id : LocalDefId , n_tps : usize , n_lts : usize , n_cts : usize , sig : ty :: PolyFnSig < 'tcx > ,) { let (generics , span) = match tcx . hir_node_by_def_id (def_id) { hir :: Node :: Item (hir :: Item { kind : hir :: ItemKind :: Fn { generics , .. } , .. }) => { (tcx . generics_of (def_id) , generics . span) } _ => tcx . dcx () . span_bug (span , "intrinsic must be a function") , } ; let own_counts = generics . own_counts () ; let gen_count_ok = | found : usize , expected : usize , descr : & str | -> bool { if found != expected { tcx . dcx () . emit_err (WrongNumberOfGenericArgumentsToIntrinsic { span , found , expected , descr , }) ; false } else { true } } ; if gen_count_ok (own_counts . lifetimes , n_lts , "lifetime") && gen_count_ok (own_counts . types , n_tps , "type") && gen_count_ok (own_counts . consts , n_cts , "const") { let _ = check_function_signature (tcx , ObligationCause :: new (span , def_id , ObligationCauseCode :: IntrinsicType) , def_id . into () , sig ,) ; } }
};
}
