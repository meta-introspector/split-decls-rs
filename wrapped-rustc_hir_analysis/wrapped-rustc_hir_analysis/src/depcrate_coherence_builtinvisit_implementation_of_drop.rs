// Generated macro for visit_implementation_of_drop (function)
macro_rules! Depcrate_coherence_builtinvisit_implementation_of_drop {
() => {
// Module: crate::coherence::builtin
// Provides: {"visit_implementation_of_drop"}
// Dependencies: {}
fn visit_implementation_of_drop (checker : & Checker < '_ >) -> Result < () , ErrorGuaranteed > { let tcx = checker . tcx ; let impl_did = checker . impl_def_id ; match checker . impl_header . trait_ref . instantiate_identity () . self_ty () . kind () { ty :: Adt (def , _) if def . did () . is_local () => return Ok (()) , ty :: Error (_) => return Ok (()) , _ => { } } let impl_ = tcx . hir_expect_item (impl_did) . expect_impl () ; Err (tcx . dcx () . emit_err (errors :: DropImplOnWrongItem { span : impl_ . self_ty . span , trait_ : tcx . item_name (checker . impl_header . trait_ref . skip_binder () . def_id) , })) }
};
}
