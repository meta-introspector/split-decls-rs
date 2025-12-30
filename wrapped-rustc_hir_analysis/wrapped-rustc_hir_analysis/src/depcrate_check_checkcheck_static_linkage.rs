// Generated macro for check_static_linkage (function)
macro_rules! Depcrate_check_checkcheck_static_linkage {
() => {
// Module: crate::check::check
// Provides: {"check_static_linkage"}
// Dependencies: {}
fn check_static_linkage (tcx : TyCtxt < '_ > , def_id : LocalDefId) { if tcx . codegen_fn_attrs (def_id) . import_linkage . is_some () { if match tcx . type_of (def_id) . instantiate_identity () . kind () { ty :: RawPtr (_ , _) => false , ty :: Adt (adt_def , args) => ! is_enum_of_nonnullable_ptr (tcx , * adt_def , * args) , _ => true , } { tcx . dcx () . emit_err (errors :: LinkageType { span : tcx . def_span (def_id) }) ; } } }
};
}
