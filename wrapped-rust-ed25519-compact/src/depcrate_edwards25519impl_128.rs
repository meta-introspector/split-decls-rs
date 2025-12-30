// Generated macro for impl_128 (impl)
macro_rules! Depcrate_edwards25519impl_128 {
() => {
// Module: crate::edwards25519
// Provides: {"impl_128"}
// Dependencies: {}
impl GeCached { pub fn maybe_set (& mut self , other : & GeCached , do_swap : u8) { self . y_plus_x . maybe_set (& other . y_plus_x , do_swap) ; self . y_minus_x . maybe_set (& other . y_minus_x , do_swap) ; self . z . maybe_set (& other . z , do_swap) ; self . t2d . maybe_set (& other . t2d , do_swap) ; } }
};
}
