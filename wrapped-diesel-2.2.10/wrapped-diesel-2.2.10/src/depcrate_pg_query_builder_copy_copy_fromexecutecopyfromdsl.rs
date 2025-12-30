// Generated macro for ExecuteCopyFromDsl (trait)
macro_rules! Depcrate_pg_query_builder_copy_copy_fromExecuteCopyFromDsl {
() => {
// Module: crate::pg::query_builder::copy::copy_from
// Provides: {"ExecuteCopyFromDsl"}
// Dependencies: {}
# [doc = " A custom execute function tailored for `COPY FROM` statements"] # [doc = ""] # [doc = " This trait can be used to execute `COPY FROM` queries constructed"] # [doc = " via [`copy_from]`"] pub trait ExecuteCopyFromDsl < C > where C : Connection < Backend = Pg > , { # [doc = " The error type returned by the execute function"] type Error : std :: error :: Error ; # [doc = " See the trait documentation for details"] fn execute (self , conn : & mut C) -> Result < usize , Self :: Error > ; }
};
}
