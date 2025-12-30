// Generated macro for impl_1440 (impl)
macro_rules! Depcrate_stringimpl_1440 {
() => {
// Module: crate::string
// Provides: {"impl_1440"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "herd_cows" , since = "1.19.0")] impl < 'a > FromIterator < Cow < 'a , str > > for String { fn from_iter < I : IntoIterator < Item = Cow < 'a , str > > > (iter : I) -> String { let mut iterator = iter . into_iter () ; match iterator . next () { None => String :: new () , Some (cow) => { let mut buf = cow . into_owned () ; buf . extend (iterator) ; buf } } } }
};
}
