// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
impl < T : Copy > AtomicLazyCell < T > { # [doc = " Returns a copy of the contents of the lazy cell."] # [doc = ""] # [doc = " This function will return `Some` if the cell has been previously initialized,"] # [doc = " and `None` if it has not yet been initialized."] pub fn get (& self) -> Option < T > { match self . state . load (Ordering :: Acquire) { SOME => unsafe { * self . inner . get () } , _ => None , } } }
};
}
