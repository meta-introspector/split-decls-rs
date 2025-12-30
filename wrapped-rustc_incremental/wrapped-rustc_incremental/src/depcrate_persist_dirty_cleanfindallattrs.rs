// Generated macro for FindAllAttrs (struct)
macro_rules! Depcrate_persist_dirty_cleanFindAllAttrs {
() => {
// Module: crate::persist::dirty_clean
// Provides: {"FindAllAttrs"}
// Dependencies: {}
# [doc = " A visitor that collects all `#[rustc_clean]` attributes from"] # [doc = " the HIR. It is used to verify that we really ran checks for all annotated"] # [doc = " nodes."] struct FindAllAttrs < 'tcx > { tcx : TyCtxt < 'tcx > , found_attrs : Vec < & 'tcx Attribute > , }
};
}
