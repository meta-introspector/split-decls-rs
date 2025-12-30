// Generated macro for swap_indices (function)
macro_rules! Depcrate_utilswap_indices {
() => {
// Module: crate::util
// Provides: {"swap_indices"}
// Dependencies: {}
# [doc = " Swap two values of anything implementing `IndexMut`."] # [doc = ""] # [doc = " Like `slice::swap`, but more generic."] # [allow (unsafe_code)] pub (crate) fn swap_indices < V > (vector : & mut V , a : usize , b : usize) where V : IndexMut < usize > , V :: Output : Sized , { if a == b { return ; } let pa : * mut V :: Output = & mut vector [a] ; let pb : * mut V :: Output = & mut vector [b] ; unsafe { ptr :: swap (pa , pb) ; } }
};
}
