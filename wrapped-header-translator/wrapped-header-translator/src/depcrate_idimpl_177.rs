// Generated macro for impl_177 (impl)
macro_rules! Depcrate_idimpl_177 {
() => {
// Module: crate::id
// Provides: {"impl_177"}
// Dependencies: {}
impl < N : ToOptionString + Ord > Ord for ItemIdentifier < N > { fn cmp (& self , other : & Self) -> Ordering { self . name . cmp (& other . name) } }
};
}
