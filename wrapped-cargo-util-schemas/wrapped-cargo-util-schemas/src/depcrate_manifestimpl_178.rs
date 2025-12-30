// Generated macro for impl_178 (impl)
macro_rules! Depcrate_manifestimpl_178 {
() => {
// Module: crate::manifest
// Provides: {"impl_178"}
// Dependencies: {}
impl fmt :: Display for ProfilePackageSpec { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ProfilePackageSpec :: Spec (spec) => spec . fmt (f) , ProfilePackageSpec :: All => f . write_str ("*") , } } }
};
}
