// Generated macro for impl_11 (impl)
macro_rules! Depcrate_dirimpl_11 {
() => {
// Module: crate::dir
// Provides: {"impl_11"}
// Dependencies: {}
impl < 'a > IgnoreMatch < 'a > { fn overrides (x : overrides :: Glob < 'a >) -> IgnoreMatch < 'a > { IgnoreMatch (IgnoreMatchInner :: Override (x)) } fn gitignore (x : & 'a gitignore :: Glob) -> IgnoreMatch < 'a > { IgnoreMatch (IgnoreMatchInner :: Gitignore (x)) } fn types (x : types :: Glob < 'a >) -> IgnoreMatch < 'a > { IgnoreMatch (IgnoreMatchInner :: Types (x)) } fn hidden () -> IgnoreMatch < 'static > { IgnoreMatch (IgnoreMatchInner :: Hidden) } }
};
}
