// Generated macro for impl_12 (impl)
macro_rules! Depcrate_argsimpl_12 {
() => {
// Module: crate::args
// Provides: {"impl_12"}
// Dependencies: {}
impl FromMeta for DefaultValue { fn from_word () -> darling :: Result < Self > { Ok (DefaultValue :: Default) } fn from_value (value : & Lit) -> darling :: Result < Self > { Ok (DefaultValue :: Value (value . clone ())) } }
};
}
