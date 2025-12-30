// Generated macro for span_mirbug (macro)
macro_rules! Depcrate_type_checkspan_mirbug {
() => {
// Module: crate::type_check
// Provides: {"span_mirbug"}
// Dependencies: {}
macro_rules ! span_mirbug { ($ context : expr , $ elem : expr , $ ($ message : tt) *) => ({ $ crate :: type_check :: mirbug ($ context . tcx () , $ context . last_span , format ! ("broken MIR in {:?} ({:?}): {}" , $ context . body () . source . def_id () , $ elem , format_args ! ($ ($ message) *) ,) ,) }) }
};
}
