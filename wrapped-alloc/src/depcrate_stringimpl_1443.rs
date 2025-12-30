// Generated macro for impl_1443 (impl)
macro_rules! Depcrate_stringimpl_1443 {
() => {
// Module: crate::string
// Provides: {"impl_1443"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl < 'a > Extend < & 'a str > for String { fn extend < I : IntoIterator < Item = & 'a str > > (& mut self , iter : I) { iter . into_iter () . for_each (move | s | self . push_str (s)) ; } # [inline] fn extend_one (& mut self , s : & 'a str) { self . push_str (s) ; } }
};
}
