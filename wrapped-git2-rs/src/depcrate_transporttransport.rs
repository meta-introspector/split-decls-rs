// Generated macro for Transport (struct)
macro_rules! Depcrate_transportTransport {
() => {
// Module: crate::transport
// Provides: {"Transport"}
// Dependencies: {}
# [doc = " A transport is a structure which knows how to transfer data to and from a"] # [doc = " remote."] # [doc = ""] # [doc = " This transport is a representation of the raw transport underneath it, which"] # [doc = " is similar to a trait object in Rust."] # [allow (missing_copy_implementations)] pub struct Transport { raw : * mut raw :: git_transport , owned : bool , }
};
}
