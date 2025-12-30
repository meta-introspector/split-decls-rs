// Generated macro for ConnectError (enum)
macro_rules! Depcrate_connect_errorConnectError {
() => {
// Module: crate::connect::error
// Provides: {"ConnectError"}
// Dependencies: {}
# [doc = " Errors that can result from using a connector service."] # [derive (Debug)] pub enum ConnectError { # [doc = " Failed to resolve the hostname."] Resolver (Box < dyn std :: error :: Error >) , # [doc = " No DNS records."] NoRecords , # [doc = " Invalid input."] InvalidInput , # [doc = " Unresolved host name."] Unresolved , # [doc = " Connection IO error."] Io (io :: Error) , }
};
}
