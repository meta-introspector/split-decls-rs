// Generated macro for impl_18 (impl)
macro_rules! Depcrate_loggerimpl_18 {
() => {
// Module: crate::logger
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'a , T > From < T > for Env < 'a > where T : Into < Cow < 'a , str > > , { fn from (filter_env : T) -> Self { Env :: default () . filter (filter_env . into ()) } }
};
}
