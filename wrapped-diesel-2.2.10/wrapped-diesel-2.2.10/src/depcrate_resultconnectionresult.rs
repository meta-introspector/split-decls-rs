// Generated macro for ConnectionResult (type)
macro_rules! Depcrate_resultConnectionResult {
() => {
// Module: crate::result
// Provides: {"ConnectionResult"}
// Dependencies: {}
# [doc = " A specialized result type for establishing connections."] # [doc = ""] # [doc = " This type exists to avoid writing out `diesel::result::ConnectionError`, and"] # [doc = " is otherwise a direct mapping to `Result`."] pub type ConnectionResult < T > = Result < T , ConnectionError > ;
};
}
