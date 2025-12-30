// Generated macro for impl_36 (impl)
macro_rules! Depcrate_file_commitimpl_36 {
() => {
// Module: crate::file::commit
// Provides: {"impl_36"}
// Dependencies: {}
impl ExtraEdge { pub fn from_raw (raw : u32) -> Self { if raw & LAST_EXTENDED_EDGE_MASK != 0 { Self :: Last (Position (raw & ! LAST_EXTENDED_EDGE_MASK)) } else { Self :: Internal (Position (raw)) } } }
};
}
