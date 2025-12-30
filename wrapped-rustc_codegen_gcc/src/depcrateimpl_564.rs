// Generated macro for impl_564 (impl)
macro_rules! Depcrateimpl_564 {
() => {
// Module: crate
// Provides: {"impl_564"}
// Dependencies: {}
impl Debug for LockedTargetInfo { fn fmt (& self , formatter : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . info . lock () . expect ("lock") . fmt (formatter) } }
};
}
