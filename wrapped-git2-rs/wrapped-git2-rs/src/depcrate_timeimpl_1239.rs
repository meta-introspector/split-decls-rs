// Generated macro for impl_1239 (impl)
macro_rules! Depcrate_timeimpl_1239 {
() => {
// Module: crate::time
// Provides: {"impl_1239"}
// Dependencies: {}
impl Ord for IndexTime { fn cmp (& self , other : & IndexTime) -> Ordering { let me = (self . raw . seconds , self . raw . nanoseconds) ; let other = (other . raw . seconds , other . raw . nanoseconds) ; me . cmp (& other) } }
};
}
