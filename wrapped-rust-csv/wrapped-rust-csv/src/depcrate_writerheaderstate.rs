// Generated macro for HeaderState (enum)
macro_rules! Depcrate_writerHeaderState {
() => {
// Module: crate::writer
// Provides: {"HeaderState"}
// Dependencies: {}
# [doc = " HeaderState encodes a small state machine for handling header writes."] # [derive (Debug)] enum HeaderState { # [doc = " Indicates that we should attempt to write a header."] Write , # [doc = " Indicates that writing a header was attempted, and a header was written."] DidWrite , # [doc = " Indicates that writing a header was attempted, but no headers were"] # [doc = " written or the attempt failed."] DidNotWrite , # [doc = " This state is used when headers are disabled. It cannot transition"] # [doc = " to any other state."] None , }
};
}
