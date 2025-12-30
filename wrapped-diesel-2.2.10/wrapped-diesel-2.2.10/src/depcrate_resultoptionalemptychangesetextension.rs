// Generated macro for OptionalEmptyChangesetExtension (trait)
macro_rules! Depcrate_resultOptionalEmptyChangesetExtension {
() => {
// Module: crate::result
// Provides: {"OptionalEmptyChangesetExtension"}
// Dependencies: {}
# [doc = " See the [method documentation](OptionalEmptyChangesetExtension::optional_empty_changeset)."] pub trait OptionalEmptyChangesetExtension < T > { # [doc = " By default, Diesel treats an empty update as a `QueryBuilderError`. This method will"] # [doc = " convert that error into `None`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use diesel::{QueryResult, OptionalEmptyChangesetExtension, result::Error::QueryBuilderError, result::EmptyChangeset};"] # [doc = " let result: QueryResult<i32> = Err(QueryBuilderError(Box::new(EmptyChangeset)));"] # [doc = " assert_eq!(Ok(None), result.optional_empty_changeset());"] # [doc = " ```"] fn optional_empty_changeset (self) -> Result < Option < T > , Error > ; }
};
}
