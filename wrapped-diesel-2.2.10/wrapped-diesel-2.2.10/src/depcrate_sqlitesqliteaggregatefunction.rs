// Generated macro for SqliteAggregateFunction (trait)
macro_rules! Depcrate_sqliteSqliteAggregateFunction {
() => {
// Module: crate::sqlite
// Provides: {"SqliteAggregateFunction"}
// Dependencies: {}
# [doc = " Trait for the implementation of a SQLite aggregate function"] # [doc = ""] # [doc = " This trait is to be used in conjunction with the `define_sql_function!`"] # [doc = " macro for defining a custom SQLite aggregate function. See"] # [doc = " the documentation [there](super::prelude::define_sql_function!) for details."] pub trait SqliteAggregateFunction < Args > : Default { # [doc = " The result type of the SQLite aggregate function"] type Output ; # [doc = " The `step()` method is called once for every record of the query."] # [doc = ""] # [doc = " This is called through a C FFI, as such panics do not propagate to the caller. Panics are"] # [doc = " caught and cause a return with an error value. The implementation must still ensure that"] # [doc = " state remains in a valid state (refer to [`std::panic::UnwindSafe`] for a bit more detail)."] fn step (& mut self , args : Args) ; # [doc = " After the last row has been processed, the `finalize()` method is"] # [doc = " called to compute the result of the aggregate function. If no rows"] # [doc = " were processed `aggregator` will be `None` and `finalize()` can be"] # [doc = " used to specify a default result."] # [doc = ""] # [doc = " This is called through a C FFI, as such panics do not propagate to the caller. Panics are"] # [doc = " caught and cause a return with an error value."] fn finalize (aggregator : Option < Self >) -> Self :: Output ; }
};
}
