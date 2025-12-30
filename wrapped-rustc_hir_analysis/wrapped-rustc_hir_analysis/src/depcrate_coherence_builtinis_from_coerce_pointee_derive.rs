// Generated macro for is_from_coerce_pointee_derive (function)
macro_rules! Depcrate_coherence_builtinis_from_coerce_pointee_derive {
() => {
// Module: crate::coherence::builtin
// Provides: {"is_from_coerce_pointee_derive"}
// Dependencies: {}
fn is_from_coerce_pointee_derive (tcx : TyCtxt < '_ > , span : Span) -> bool { span . ctxt () . outer_expn_data () . macro_def_id . is_some_and (| def_id | tcx . is_diagnostic_item (sym :: CoercePointee , def_id)) }
};
}
