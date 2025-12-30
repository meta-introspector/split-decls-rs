// Generated macro for Error (enum)
macro_rules! Depcrate_r2d2Error {
() => {
// Module: crate::r2d2
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error used when managing connections with `r2d2`."] # [derive (Debug)] pub enum Error { # [doc = " An error occurred establishing the connection"] ConnectionError (ConnectionError) , # [doc = " An error occurred pinging the database"] QueryError (crate :: result :: Error) , }
};
}
