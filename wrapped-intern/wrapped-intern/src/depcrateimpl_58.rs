// Generated macro for impl_58 (impl)
macro_rules! Depcrateimpl_58 {
() => {
// Module: crate
// Provides: {"impl_58"}
// Dependencies: {}
impl < T : Internable + ? Sized > InternStorage < T > { fn get (& self) -> & InternMap < T > { self . map . get_or_init (DashMap :: default) } }
};
}
