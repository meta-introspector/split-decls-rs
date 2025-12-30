// Generated macro for IntoUpdateTarget (trait)
macro_rules! Depcrate_query_builder_update_statement_targetIntoUpdateTarget {
() => {
// Module: crate::query_builder::update_statement::target
// Provides: {"IntoUpdateTarget"}
// Dependencies: {}
# [doc = " A type which can be passed to [`update`] or [`delete`]."] # [doc = ""] # [doc = " Apps will never need to implement this type directly. There are three kinds"] # [doc = " which implement this trait. Tables, queries which have only had `filter`"] # [doc = " called on them, and types which implement `Identifiable`."] # [doc = ""] # [doc = " When a table is passed to `update`, every row in the table will be updated."] # [doc = " You can scope this down by calling [`filter`] which will"] # [doc = " result in `UPDATE your_table SET ... WHERE args_to_filter`. Passing a type"] # [doc = " which implements `Identifiable` is the same as passing"] # [doc = " `SomeStruct::table().find(some_struct)`."] # [doc = ""] # [doc = " [`update`]: crate::update()"] # [doc = " [`delete`]: crate::delete()"] # [doc = " [`filter`]: crate::query_builder::UpdateStatement::filter()"] pub trait IntoUpdateTarget : HasTable { # [doc = " What is the `WHERE` clause of this target?"] type WhereClause ; # [doc = " Decomposes `self` into the table and where clause."] fn into_update_target (self) -> UpdateTarget < Self :: Table , Self :: WhereClause > ; }
};
}
