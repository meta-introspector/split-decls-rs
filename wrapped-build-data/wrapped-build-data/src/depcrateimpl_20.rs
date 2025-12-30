// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl core :: fmt :: Display for RustChannel { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> Result < () , core :: fmt :: Error > { match self { RustChannel :: Stable => write ! (f , "stable") , RustChannel :: Beta => write ! (f , "beta") , RustChannel :: Nightly => write ! (f , "nightly") , } } }
};
}
