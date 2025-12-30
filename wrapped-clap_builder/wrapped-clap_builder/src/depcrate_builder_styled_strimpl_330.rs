// Generated macro for impl_330 (impl)
macro_rules! Depcrate_builder_styled_strimpl_330 {
() => {
// Module: crate::builder::styled_str
// Provides: {"impl_330"}
// Dependencies: {}
# [doc = " Color-unaware printing. Never uses coloring."] impl std :: fmt :: Display for StyledStr { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { for part in self . iter_text () { part . fmt (f) ? ; } Ok (()) } }
};
}
