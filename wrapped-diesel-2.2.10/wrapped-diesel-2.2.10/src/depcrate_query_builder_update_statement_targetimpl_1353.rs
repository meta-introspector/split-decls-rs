// Generated macro for impl_1353 (impl)
macro_rules! Depcrate_query_builder_update_statement_targetimpl_1353 {
() => {
// Module: crate::query_builder::update_statement::target
// Provides: {"impl_1353"}
// Dependencies: {}
impl < T , Tab , V > IntoUpdateTarget for T where T : Identifiable < Table = Tab > , Tab : Table + FindDsl < T :: Id > , Find < Tab , T :: Id > : IntoUpdateTarget < Table = Tab , WhereClause = V > , { type WhereClause = V ; fn into_update_target (self) -> UpdateTarget < Self :: Table , Self :: WhereClause > { T :: table () . find (self . id ()) . into_update_target () } }
};
}
