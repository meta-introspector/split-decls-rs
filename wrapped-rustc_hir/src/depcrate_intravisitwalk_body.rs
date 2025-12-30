// Generated macro for walk_body (function)
macro_rules! Depcrate_intravisitwalk_body {
() => {
// Module: crate::intravisit
// Provides: {"walk_body"}
// Dependencies: {}
pub fn walk_body < 'v , V : Visitor < 'v > > (visitor : & mut V , body : & Body < 'v >) -> V :: Result { let Body { params , value } = body ; walk_list ! (visitor , visit_param , * params) ; visitor . visit_expr (* value) }
};
}
