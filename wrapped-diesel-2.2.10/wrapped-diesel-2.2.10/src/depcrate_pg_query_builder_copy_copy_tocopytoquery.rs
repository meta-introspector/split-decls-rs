// Generated macro for CopyToQuery (struct)
macro_rules! Depcrate_pg_query_builder_copy_copy_toCopyToQuery {
() => {
// Module: crate::pg::query_builder::copy::copy_to
// Provides: {"CopyToQuery"}
// Dependencies: {}
# [doc = " The structure returned by [`copy_to`]"] # [doc = ""] # [doc = " The [`load`] and the [`load_raw`] methods allow"] # [doc = " to receive the configured data from the database."] # [doc = " If you don't have any special needs you should prefer using"] # [doc = " the more convenient `load` method."] # [doc = ""] # [doc = " The `with_*` methods allow to configure the settings used for the"] # [doc = " copy statement."] # [doc = ""] # [doc = " [`load`]: CopyToQuery::load"] # [doc = " [`load_raw`]: CopyToQuery::load_raw"] # [derive (Debug)] # [must_use = "`COPY TO` statements are only executed when calling `.load()` or `load_raw()`."] # [cfg (feature = "postgres_backend")] pub struct CopyToQuery < T , O > { target : T , options : O , }
};
}
