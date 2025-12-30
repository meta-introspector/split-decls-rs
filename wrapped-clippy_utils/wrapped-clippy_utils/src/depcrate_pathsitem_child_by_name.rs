// Generated macro for item_child_by_name (function)
macro_rules! Depcrate_pathsitem_child_by_name {
() => {
// Module: crate::paths
// Provides: {"item_child_by_name"}
// Dependencies: {}
fn item_child_by_name (tcx : TyCtxt < '_ > , def_id : DefId , ns : PathNS , name : Symbol) -> Option < DefId > { if let Some (local_id) = def_id . as_local () { local_item_child_by_name (tcx , local_id , ns , name) } else { non_local_item_child_by_name (tcx , def_id , ns , name) } }
};
}
