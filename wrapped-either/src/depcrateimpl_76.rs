// Generated macro for impl_76 (impl)
macro_rules! Depcrateimpl_76 {
() => {
// Module: crate
// Provides: {"impl_76"}
// Dependencies: {}
impl < L , R > fmt :: Display for Either < L , R > where L : fmt :: Display , R : fmt :: Display , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for_both ! (self , inner => inner . fmt (f)) } }
};
}
