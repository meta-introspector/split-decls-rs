// Generated macro for StreamIdHasher (struct)
macro_rules! Depcrate_streamStreamIdHasher {
() => {
// Module: crate::stream
// Provides: {"StreamIdHasher"}
// Dependencies: {}
# [doc = " A simple no-op hasher for Stream IDs."] # [doc = ""] # [doc = " The QUIC protocol and quiche library guarantees stream ID uniqueness, so"] # [doc = " we can save effort by avoiding using a more complicated algorithm."] # [derive (Default)] pub struct StreamIdHasher { id : u64 , }
};
}
