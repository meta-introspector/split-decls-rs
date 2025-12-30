// Generated macro for POSTGRES_EPOCH_DATE (static)
macro_rules! Depcrate_postgresPOSTGRES_EPOCH_DATE {
() => {
// Module: crate::postgres
// Provides: {"POSTGRES_EPOCH_DATE"}
// Dependencies: {}
# [doc = " Apparently the actual format of values on the wire is not"] # [doc = " a documented guarantee of PostgreSQL.[1] Instead, I just `sqlx`'s"] # [doc = " source code for `chrono` to figure out what the type of the source"] # [doc = " data is."] # [doc = ""] # [doc = " [1]: https://www.postgresql.org/docs/current/protocol-overview.html#PROTOCOL-FORMAT-CODES"] static POSTGRES_EPOCH_DATE : civil :: Date = civil :: date (2000 , 1 , 1) ;
};
}
