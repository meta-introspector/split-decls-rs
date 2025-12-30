// Generated macro for impl_59 (impl)
macro_rules! Depcrate_sizeimpl_59 {
() => {
// Module: crate::size
// Provides: {"impl_59"}
// Dependencies: {}
impl PartialOrd for KeySize { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { let l = self . l_aligned () . partial_cmp (& other . l_aligned ()) ? ; let n = self . n_aligned () . partial_cmp (& other . n_aligned ()) ? ; Some (l . then (n)) } }
};
}
