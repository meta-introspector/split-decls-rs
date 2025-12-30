// Generated macro for ConnectionState (enum)
macro_rules! Depcrate_graphql_wsConnectionState {
() => {
// Module: crate::graphql_ws
// Provides: {"ConnectionState"}
// Dependencies: {}
enum ConnectionState < S : Schema , I : Init < S :: ScalarValue , S :: Context > > { # [doc = " PreInit is the state before a ConnectionInit message has been accepted."] PreInit { init : I , schema : S } , # [doc = " Active is the state after a ConnectionInit message has been accepted."] Active { config : Arc < ConnectionConfig < S :: Context > > , stoppers : HashMap < String , oneshot :: Sender < () > > , schema : S , } , # [doc = " Terminated is the state after a ConnectionInit message has been rejected."] Terminated , }
};
}
