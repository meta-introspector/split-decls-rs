// Generated macro for InFlightData (enum)
macro_rules! Depcrate_proto_streams_prioritizeInFlightData {
() => {
// Module: crate::proto::streams::prioritize
// Provides: {"InFlightData"}
// Dependencies: {}
# [derive (Debug , Eq , PartialEq)] enum InFlightData { # [doc = " There is no `DATA` frame in flight."] Nothing , # [doc = " There is a `DATA` frame in flight belonging to the given stream."] DataFrame (store :: Key) , # [doc = " There was a `DATA` frame, but the stream's queue was since cleared."] Drop , }
};
}
