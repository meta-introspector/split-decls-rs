// Generated macro for impl_76 (impl)
macro_rules! Depcrateimpl_76 {
() => {
// Module: crate
// Provides: {"impl_76"}
// Dependencies: {}
impl < T : Clone + Signed > Complex < T > { # [doc = " Returns the L1 norm `|re| + |im|` -- the [Manhattan distance] from the origin."] # [doc = ""] # [doc = " [Manhattan distance]: https://en.wikipedia.org/wiki/Taxicab_geometry"] # [inline] pub fn l1_norm (& self) -> T { self . re . abs () + self . im . abs () } }
};
}
