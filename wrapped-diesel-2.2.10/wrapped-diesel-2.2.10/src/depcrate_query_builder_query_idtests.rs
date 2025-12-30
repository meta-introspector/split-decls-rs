// Generated macro for tests (module)
macro_rules! Depcrate_query_builder_query_idtests {
() => {
// Module: crate::query_builder::query_id
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [allow (unused_parens)] mod tests { use std :: any :: TypeId ; use super :: QueryId ; use crate :: prelude :: * ; table ! { users { id -> Integer , name -> VarChar , } } fn query_id < T : QueryId > (_ : T) -> Option < TypeId > { T :: query_id () } # [test] fn queries_with_no_dynamic_elements_have_a_static_id () { use self :: users :: dsl :: * ; assert ! (query_id (users) . is_some ()) ; assert ! (query_id (users . select (name)) . is_some ()) ; assert ! (query_id (users . filter (name . eq ("Sean"))) . is_some ()) ; } # [test] fn queries_with_different_types_have_different_ids () { let id1 = query_id (users :: table . select (users :: name)) ; let id2 = query_id (users :: table . select (users :: id)) ; assert_ne ! (id1 , id2) ; } # [test] fn bind_params_use_only_sql_type_for_query_id () { use self :: users :: dsl :: * ; let id1 = query_id (users . filter (name . eq ("Sean"))) ; let id2 = query_id (users . filter (name . eq ("Tess" . to_string ()))) ; assert_eq ! (id1 , id2) ; } # [test] # [cfg (feature = "postgres")] fn boxed_queries_do_not_have_static_query_id () { use crate :: pg :: Pg ; assert ! (query_id (users :: table . into_boxed ::< Pg > ()) . is_none ()) ; } }
};
}
