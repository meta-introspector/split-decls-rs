// Generated macro for OptionalExtension (trait)
macro_rules! DepcrateOptionalExtension {
() => {
// Module: crate
// Provides: {"OptionalExtension"}
// Dependencies: {}
# [doc = " See the [method documentation](#tymethod.optional)."] pub trait OptionalExtension < T > { # [doc = " Converts a `Result<T>` into a `Result<Option<T>>`."] # [doc = ""] # [doc = " By default, Rusqlite treats 0 rows being returned from a query that is"] # [doc = " expected to return 1 row as an error. This method will"] # [doc = " handle that error, and give you back an `Option<T>` instead."] fn optional (self) -> Result < Option < T > > ; }
};
}
