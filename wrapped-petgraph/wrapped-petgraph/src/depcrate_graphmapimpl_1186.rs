// Generated macro for impl_1186 (impl)
macro_rules! Depcrate_graphmapimpl_1186 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1186"}
// Dependencies: {}
impl CompactDirection { # [doc = " Return the opposite `CompactDirection`."] # [inline] pub fn opposite (self) -> CompactDirection { match self { CompactDirection :: Outgoing => CompactDirection :: Incoming , CompactDirection :: Incoming => CompactDirection :: Outgoing , } } }
};
}
