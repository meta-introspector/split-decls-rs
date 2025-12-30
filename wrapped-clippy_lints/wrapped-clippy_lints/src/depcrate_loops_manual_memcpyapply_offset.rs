// Generated macro for apply_offset (function)
macro_rules! Depcrate_loops_manual_memcpyapply_offset {
() => {
// Module: crate::loops::manual_memcpy
// Provides: {"apply_offset"}
// Dependencies: {}
fn apply_offset (lhs : & MinifyingSugg < 'static > , rhs : & Offset) -> MinifyingSugg < 'static > { match rhs . sign { OffsetSign :: Positive => lhs + & rhs . value , OffsetSign :: Negative => lhs - & rhs . value , } }
};
}
