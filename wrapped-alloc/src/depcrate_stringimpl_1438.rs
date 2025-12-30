// Generated macro for impl_1438 (impl)
macro_rules! Depcrate_stringimpl_1438 {
() => {
// Module: crate::string
// Provides: {"impl_1438"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "extend_string" , since = "1.4.0")] impl FromIterator < String > for String { fn from_iter < I : IntoIterator < Item = String > > (iter : I) -> String { let mut iterator = iter . into_iter () ; match iterator . next () { None => String :: new () , Some (mut buf) => { buf . extend (iterator) ; buf } } } }
};
}
