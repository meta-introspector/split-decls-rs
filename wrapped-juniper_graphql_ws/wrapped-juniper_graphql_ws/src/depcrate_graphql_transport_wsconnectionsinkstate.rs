// Generated macro for ConnectionSinkState (enum)
macro_rules! Depcrate_graphql_transport_wsConnectionSinkState {
() => {
// Module: crate::graphql_transport_ws
// Provides: {"ConnectionSinkState"}
// Dependencies: {}
enum ConnectionSinkState < S : Schema , I : Init < S :: ScalarValue , S :: Context > > { Ready { state : ConnectionState < S , I > , } , HandlingMessage { # [expect (clippy :: type_complexity , reason = "not really")] result : BoxFuture < 'static , (ConnectionState < S , I > , BoxStream < 'static , Output < S :: ScalarValue > > ,) , > , } , Closed , }
};
}
