// Generated macro for impl_9 (impl)
macro_rules! Depcrate_helpersimpl_9 {
() => {
// Module: crate::helpers
// Provides: {"impl_9"}
// Dependencies: {}
impl ToDep for & 'static str { fn to_dep (self) -> Dependency { Dependency :: parse (self , Some ("1.0.0") , registry_loc ()) . unwrap () } fn opt (self) -> Dependency { let mut dep = self . to_dep () ; dep . set_optional (true) ; dep } fn with (self , features : & [& 'static str]) -> Dependency { let mut dep = self . to_dep () ; dep . set_default_features (false) ; dep . set_features (features . into_iter () . copied ()) ; dep } fn with_default (self) -> Dependency { let mut dep = self . to_dep () ; dep . set_default_features (true) ; dep } fn rename (self , name : & str) -> Dependency { let mut dep = self . to_dep () ; dep . set_explicit_name_in_toml (name) ; dep } }
};
}
