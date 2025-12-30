// Generated macro for ServerError (struct)
macro_rules! DepcrateServerError {
() => {
// Module: crate
// Provides: {"ServerError"}
// Dependencies: {}
# [doc = " Represents errors encountered when communicating with the proc-macro server."] # [derive (Clone , Debug)] pub struct ServerError { pub message : String , pub io : Option < Arc < io :: Error > > , }
};
}
