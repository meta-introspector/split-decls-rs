// Generated macro for State (enum)
macro_rules! Depcrate_eventsourceState {
() => {
// Module: crate::eventsource
// Provides: {"State"}
// Dependencies: {}
# [doc = " The state of the EventSource."] # [doc = ""] # [doc = " See [`EventSource.readyState` on MDN](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/readyState)"] # [doc = " to learn more."] # [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum State { # [doc = " The connection has not yet been established."] Connecting , # [doc = " The EventSource connection is established and communication is possible."] Open , # [doc = " The connection has been closed or could not be opened."] Closed , }
};
}
