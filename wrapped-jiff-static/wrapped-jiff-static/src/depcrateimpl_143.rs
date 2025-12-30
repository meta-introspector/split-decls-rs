// Generated macro for impl_143 (impl)
macro_rules! Depcrateimpl_143 {
() => {
// Module: crate
// Provides: {"impl_143"}
// Dependencies: {}
impl TzifTransitionInfo { fn quote (& self) -> proc_macro2 :: TokenStream { let TzifTransitionInfo { type_index , kind } = * self ; let kind = kind . quote () ; quote ! { jiff :: shared :: TzifTransitionInfo { type_index : # type_index , kind : # kind , } } } }
};
}
