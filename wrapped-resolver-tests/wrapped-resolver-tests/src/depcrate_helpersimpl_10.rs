// Generated macro for impl_10 (impl)
macro_rules! Depcrate_helpersimpl_10 {
() => {
// Module: crate::helpers
// Provides: {"impl_10"}
// Dependencies: {}
impl ToDep for Dependency { fn to_dep (self) -> Dependency { self } fn opt (mut self) -> Dependency { self . set_optional (true) ; self } fn with (mut self , features : & [& 'static str]) -> Dependency { self . set_default_features (false) ; self . set_features (features . into_iter () . copied ()) ; self } fn with_default (mut self) -> Dependency { self . set_default_features (true) ; self } fn rename (mut self , name : & str) -> Dependency { self . set_explicit_name_in_toml (name) ; self } }
};
}
