// Generated macro for impl_176 (impl)
macro_rules! Depcrate_idimpl_176 {
() => {
// Module: crate::id
// Provides: {"impl_176"}
// Dependencies: {}
impl < N : ToOptionString + PartialOrd > PartialOrd for ItemIdentifier < N > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . name . partial_cmp (& other . name) } }
};
}
