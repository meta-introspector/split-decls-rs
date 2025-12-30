// Generated macro for impl_189 (impl)
macro_rules! Depcrate_locationimpl_189 {
() => {
// Module: crate::location
// Provides: {"impl_189"}
// Dependencies: {}
impl Ord for LineColumn { fn cmp (& self , other : & Self) -> Ordering { self . line . cmp (& other . line) . then (self . column . cmp (& other . column)) } }
};
}
