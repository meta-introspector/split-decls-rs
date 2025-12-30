// Generated macro for OptionalExtension (trait)
macro_rules! Depcrate_resultOptionalExtension {
() => {
// Module: crate::result
// Provides: {"OptionalExtension"}
// Dependencies: {}
# [doc = " See the [method documentation](OptionalExtension::optional)."] pub trait OptionalExtension < T > { # [doc = " Converts a `QueryResult<T>` into a `QueryResult<Option<T>>`."] # [doc = ""] # [doc = " By default, Diesel treats 0 rows being returned from a query that is expected to return 1"] # [doc = " row as an error (e.g. the return value of [`get_result`] or [`first`]). This method will"] # [doc = " handle that error, and give you back an `Option<T>` instead."] # [doc = ""] # [doc = " [`get_result`]: crate::query_dsl::RunQueryDsl::get_result()"] # [doc = " [`first`]: crate::query_dsl::RunQueryDsl::first()"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use diesel::{QueryResult, NotFound, OptionalExtension};"] # [doc = ""] # [doc = " let result: QueryResult<i32> = Ok(1);"] # [doc = " assert_eq!(Ok(Some(1)), result.optional());"] # [doc = ""] # [doc = " let result: QueryResult<i32> = Err(NotFound);"] # [doc = " assert_eq!(Ok(None), result.optional());"] # [doc = " ```"] fn optional (self) -> Result < Option < T > , Error > ; }
};
}
