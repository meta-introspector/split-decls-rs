// Generated macro for impl_151 (impl)
macro_rules! Depcrateimpl_151 {
() => {
// Module: crate
// Provides: {"impl_151"}
// Dependencies: {}
impl PosixTime { fn quote (& self) -> proc_macro2 :: TokenStream { let PosixTime { second } = * self ; quote ! { jiff :: shared :: PosixTime { second : # second } } } }
};
}
