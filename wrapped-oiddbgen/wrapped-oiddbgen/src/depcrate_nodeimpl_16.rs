// Generated macro for impl_16 (impl)
macro_rules! Depcrate_nodeimpl_16 {
() => {
// Module: crate::node
// Provides: {"impl_16"}
// Dependencies: {}
impl Ord for Node { fn cmp (& self , other : & Self) -> Ordering { match self . obid . cmp (& other . obid) { Ordering :: Equal => match self . name . len () . cmp (& other . name . len ()) { Ordering :: Equal => self . name . cmp (& other . name) , o => o , } , o => o , } } }
};
}
