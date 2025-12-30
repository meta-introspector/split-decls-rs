// Generated macro for data_types (module)
macro_rules! Depcrate_pgdata_types {
() => {
// Module: crate::pg
// Provides: {"data_types"}
// Dependencies: {}
# [doc = " Data structures for PG types which have no corresponding Rust type"] # [doc = ""] # [doc = " Most of these types are used to implement `ToSql` and `FromSql` for higher"] # [doc = " level types."] pub mod data_types { # [doc (inline)] pub use super :: types :: date_and_time :: { PgDate , PgInterval , PgTime , PgTimestamp } ; # [doc (inline)] pub use super :: types :: floats :: PgNumeric ; # [doc (inline)] pub use super :: types :: money :: PgMoney ; pub use super :: types :: money :: PgMoney as Cents ; }
};
}
