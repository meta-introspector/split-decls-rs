// Generated macro for UpdateAndFetchResults (trait)
macro_rules! Depcrate_query_dsl_save_changes_dslUpdateAndFetchResults {
() => {
// Module: crate::query_dsl::save_changes_dsl
// Provides: {"UpdateAndFetchResults"}
// Dependencies: {}
# [doc = " A trait defining how to update a record and fetch the updated entry"] # [doc = " on a certain backend."] # [doc = ""] # [doc = " The only case where it is required to work with this trait is while"] # [doc = " implementing a new connection type."] # [doc = " Otherwise use [`SaveChangesDsl`]"] # [doc = ""] # [doc = " For implementing this trait for a custom backend:"] # [doc = " * The `Changes` generic parameter represents the changeset that should be stored"] # [doc = " * The `Output` generic parameter represents the type of the response."] pub trait UpdateAndFetchResults < Changes , Output > : Connection { # [doc = " See the traits documentation."] fn update_and_fetch (& mut self , changeset : Changes) -> QueryResult < Output > ; }
};
}
