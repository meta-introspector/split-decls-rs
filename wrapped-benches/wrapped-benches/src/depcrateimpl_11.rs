// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
# [graphql_object (context = Context)] impl Query { fn user_sync_instant (id : i32) -> Result < User , FieldError > { Ok (User :: new (id)) } fn users_sync_instant (ids : Option < Vec < i32 > >) -> Result < Vec < User > , FieldError > { if let Some (ids) = ids { let users = ids . into_iter () . map (User :: new) . collect () ; Ok (users) } else { Ok (vec ! []) } } async fn user_async_instant (id : i32) -> Result < User , FieldError > { Ok (User :: new (id)) } async fn users_async_instant (ids : Option < Vec < i32 > >) -> Result < Vec < User > , FieldError > { if let Some (ids) = ids { let users = ids . into_iter () . map (User :: new) . collect () ; Ok (users) } else { Ok (vec ! []) } } }
};
}
