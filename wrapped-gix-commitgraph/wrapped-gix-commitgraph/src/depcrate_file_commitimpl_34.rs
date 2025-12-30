// Generated macro for impl_34 (impl)
macro_rules! Depcrate_file_commitimpl_34 {
() => {
// Module: crate::file::commit
// Provides: {"impl_34"}
// Dependencies: {}
impl ParentEdge { pub fn from_raw (raw : u32) -> ParentEdge { if raw == NO_PARENT { return ParentEdge :: None ; } if raw & EXTENDED_EDGES_MASK != 0 { ParentEdge :: ExtraEdgeIndex (raw & ! EXTENDED_EDGES_MASK) } else { ParentEdge :: GraphPosition (Position (raw)) } } }
};
}
