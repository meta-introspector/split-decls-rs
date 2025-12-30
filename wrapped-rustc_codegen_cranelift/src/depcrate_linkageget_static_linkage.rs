// Generated macro for get_static_linkage (function)
macro_rules! Depcrate_linkageget_static_linkage {
() => {
// Module: crate::linkage
// Provides: {"get_static_linkage"}
// Dependencies: {}
pub (crate) fn get_static_linkage (tcx : TyCtxt < '_ > , def_id : DefId) -> Linkage { let fn_attrs = tcx . codegen_fn_attrs (def_id) ; if let Some (linkage) = fn_attrs . linkage { match linkage { RLinkage :: External => Linkage :: Export , RLinkage :: Internal => Linkage :: Local , RLinkage :: ExternalWeak | RLinkage :: WeakAny => Linkage :: Preemptible , _ => panic ! ("{:?}" , linkage) , } } else if tcx . is_reachable_non_generic (def_id) { Linkage :: Export } else { Linkage :: Hidden } }
};
}
