// Generated macro for walk_generics (function)
macro_rules! Depcrate_intravisitwalk_generics {
() => {
// Module: crate::intravisit
// Provides: {"walk_generics"}
// Dependencies: {}
pub fn walk_generics < 'v , V : Visitor < 'v > > (visitor : & mut V , generics : & 'v Generics < 'v >) -> V :: Result { let & Generics { params , predicates , has_where_clause_predicates : _ , where_clause_span : _ , span : _ , } = generics ; walk_list ! (visitor , visit_generic_param , params) ; walk_list ! (visitor , visit_where_predicate , predicates) ; V :: Result :: output () }
};
}
