// Generated macro for error_codes (module)
macro_rules! Depcrate_pg_connection_resulterror_codes {
() => {
// Module: crate::pg::connection::result
// Provides: {"error_codes"}
// Dependencies: {}
mod error_codes { # ! [doc = " These error codes are documented at"] # ! [doc = " <https://www.postgresql.org/docs/current/errcodes-appendix.html>"] # ! [doc = ""] # ! [doc = " They are not exposed programmatically through libpq."] pub (in crate :: pg :: connection) const CONNECTION_EXCEPTION : & str = "08000" ; pub (in crate :: pg :: connection) const CONNECTION_FAILURE : & str = "08006" ; pub (in crate :: pg :: connection) const SQLCLIENT_UNABLE_TO_ESTABLISH_SQLCONNECTION : & str = "08001" ; pub (in crate :: pg :: connection) const SQLSERVER_REJECTED_ESTABLISHMENT_OF_SQLCONNECTION : & str = "08004" ; pub (in crate :: pg :: connection) const NOT_NULL_VIOLATION : & str = "23502" ; pub (in crate :: pg :: connection) const FOREIGN_KEY_VIOLATION : & str = "23503" ; pub (in crate :: pg :: connection) const UNIQUE_VIOLATION : & str = "23505" ; pub (in crate :: pg :: connection) const CHECK_VIOLATION : & str = "23514" ; pub (in crate :: pg :: connection) const READ_ONLY_TRANSACTION : & str = "25006" ; pub (in crate :: pg :: connection) const SERIALIZATION_FAILURE : & str = "40001" ; }
};
}
