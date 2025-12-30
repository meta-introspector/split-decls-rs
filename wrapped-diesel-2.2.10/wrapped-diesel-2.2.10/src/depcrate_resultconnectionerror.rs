// Generated macro for ConnectionError (enum)
macro_rules! Depcrate_resultConnectionError {
() => {
// Module: crate::result
// Provides: {"ConnectionError"}
// Dependencies: {}
# [doc = " Errors which can occur during [`Connection::establish`]"] # [doc = ""] # [doc = " [`Connection::establish`]: crate::connection::Connection::establish"] # [derive (Debug , PartialEq)] # [non_exhaustive] pub enum ConnectionError { # [doc = " The connection URL contained a `NUL` byte."] InvalidCString (NulError) , # [doc = " The database returned an error."] BadConnection (String) , # [doc = " The connection URL could not be parsed."] InvalidConnectionUrl (String) , # [doc = " Diesel could not configure the database connection."] # [doc = ""] # [doc = " Diesel may try to automatically set session specific configuration"] # [doc = " values, such as UTF8 encoding, or enabling the `||` operator on MySQL."] # [doc = " This variant is returned if an error occurred executing the query to set"] # [doc = " those options. Diesel will never affect global configuration."] CouldntSetupConfiguration (Error) , }
};
}
