// Generated macro for Shutdown (enum)
macro_rules! DepcrateShutdown {
() => {
// Module: crate
// Provides: {"Shutdown"}
// Dependencies: {}
# [doc = " The side of the stream to be shut down."] # [doc = ""] # [doc = " This should be used when calling [`stream_shutdown()`]."] # [doc = ""] # [doc = " [`stream_shutdown()`]: struct.Connection.html#method.stream_shutdown"] # [repr (C)] # [derive (PartialEq , Eq)] pub enum Shutdown { # [doc = " Stop receiving stream data."] Read = 0 , # [doc = " Stop sending stream data."] Write = 1 , }
};
}
