// Generated macro for CopyFromQuery (struct)
macro_rules! Depcrate_pg_query_builder_copy_copy_fromCopyFromQuery {
() => {
// Module: crate::pg::query_builder::copy::copy_from
// Provides: {"CopyFromQuery"}
// Dependencies: {}
# [doc = " The structure returned by [`copy_from`]"] # [doc = ""] # [doc = " The [`from_raw_data`] and the [`from_insertable`] methods allow"] # [doc = " to configure the data copied into the database"] # [doc = ""] # [doc = " The `with_*` methods allow to configure the settings used for the"] # [doc = " copy statement."] # [doc = ""] # [doc = " [`from_raw_data`]: CopyFromQuery::from_raw_data"] # [doc = " [`from_insertable`]: CopyFromQuery::from_insertable"] # [derive (Debug)] # [must_use = "`COPY FROM` statements are only executed when calling `.execute()`."] # [cfg (feature = "postgres_backend")] pub struct CopyFromQuery < T , Action > { table : T , action : Action , }
};
}
