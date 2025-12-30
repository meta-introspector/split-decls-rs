// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl < T : Copy > LazyCell < T > { # [doc = " Returns a copy of the contents of the lazy cell."] # [doc = ""] # [doc = " This function will return `Some` if the cell has been previously initialized,"] # [doc = " and `None` if it has not yet been initialized."] pub fn get (& self) -> Option < T > { unsafe { * self . inner . get () } } }
};
}
