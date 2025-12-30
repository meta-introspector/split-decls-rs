// Generated macro for Keyed (struct)
macro_rules! Depcrate_stream_stream_groupKeyed {
() => {
// Module: crate::stream::stream_group
// Provides: {"Keyed"}
// Dependencies: {}
# [doc = " Iterate over items in the stream group with their associated keys."] # [derive (Debug)] # [pin_project :: pin_project] pub struct Keyed < S : Stream > { # [pin] group : StreamGroup < S > , }
};
}
