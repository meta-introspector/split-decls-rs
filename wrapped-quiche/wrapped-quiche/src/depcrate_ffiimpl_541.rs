// Generated macro for impl_541 (impl)
macro_rules! Depcrate_ffiimpl_541 {
() => {
// Module: crate::ffi
// Provides: {"impl_541"}
// Dependencies: {}
impl < 'a > Iterator for ConnectionIdIter < 'a > { type Item = ConnectionId < 'a > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { let v = self . cids . get (self . index) ? ; self . index += 1 ; Some (v . clone ()) } }
};
}
