// Generated macro for impl_148 (impl)
macro_rules! Depcrateimpl_148 {
() => {
// Module: crate
// Provides: {"impl_148"}
// Dependencies: {}
impl PosixRule { fn quote (& self) -> proc_macro2 :: TokenStream { let start = self . start . quote () ; let end = self . end . quote () ; quote ! { jiff :: shared :: PosixRule { start : # start , end : # end } } } }
};
}
