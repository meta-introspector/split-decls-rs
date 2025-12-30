// Generated macro for is_within_packed (function)
macro_rules! Depcrate_util_alignmentis_within_packed {
() => {
// Module: crate::util::alignment
// Provides: {"is_within_packed"}
// Dependencies: {}
pub fn is_within_packed < 'tcx , L > (tcx : TyCtxt < 'tcx > , local_decls : & L , place : Place < 'tcx > ,) -> Option < Align > where L : HasLocalDecls < 'tcx > , { place . iter_projections () . rev () . take_while (| (_base , elem) | ! matches ! (elem , ProjectionElem :: Deref)) . filter_map (| (base , _elem) | { base . ty (local_decls , tcx) . ty . ty_adt_def () . and_then (| adt | adt . repr () . pack) }) . min () }
};
}
