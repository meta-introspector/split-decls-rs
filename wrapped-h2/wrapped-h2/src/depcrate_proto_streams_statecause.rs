// Generated macro for Cause (enum)
macro_rules! Depcrate_proto_streams_stateCause {
() => {
// Module: crate::proto::streams::state
// Provides: {"Cause"}
// Dependencies: {}
# [derive (Debug , Clone)] enum Cause { EndStream , Error (Error) , # [doc = " This indicates to the connection that a reset frame must be sent out"] # [doc = " once the send queue has been flushed."] # [doc = ""] # [doc = " Examples of when this could happen:"] # [doc = " - User drops all references to a stream, so we want to CANCEL the it."] # [doc = " - Header block size was too large, so we want to REFUSE, possibly"] # [doc = "   after sending a 431 response frame."] ScheduledLibraryReset (Reason) , }
};
}
