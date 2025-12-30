// Generated macro for walk_qpath (function)
macro_rules! Depcrate_intravisitwalk_qpath {
() => {
// Module: crate::intravisit
// Provides: {"walk_qpath"}
// Dependencies: {}
pub fn walk_qpath < 'v , V : Visitor < 'v > > (visitor : & mut V , qpath : & 'v QPath < 'v > , id : HirId ,) -> V :: Result { match * qpath { QPath :: Resolved (ref maybe_qself , ref path) => { visit_opt ! (visitor , visit_ty_unambig , maybe_qself) ; visitor . visit_path (path , id) } QPath :: TypeRelative (ref qself , ref segment) => { try_visit ! (visitor . visit_ty_unambig (qself)) ; visitor . visit_path_segment (segment) } QPath :: LangItem (..) => V :: Result :: output () , } }
};
}
