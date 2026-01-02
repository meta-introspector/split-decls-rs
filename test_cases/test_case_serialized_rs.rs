// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_query_system/src/dep_graph/serialized.rs
// Error: expected square brackets
// Problematic line: line 70

// The maximum value of `SerializedDepNodeIndex` leaves the upper two bits
// unused so that we can store multiple index types in `CompressedHybridIndex`,
// and use those bits to encode which index type it contains.
rustc_index::newtype_index! {
    #[encodable]
    #[max = 0x7FFF_FFFF]
    pub struct SerializedDepNodeIndex {}
