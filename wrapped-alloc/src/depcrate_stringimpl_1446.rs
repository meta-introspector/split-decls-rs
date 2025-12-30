// Generated macro for impl_1446 (impl)
macro_rules! Depcrate_stringimpl_1446 {
() => {
// Module: crate::string
// Provides: {"impl_1446"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "herd_cows" , since = "1.19.0")] impl < 'a > Extend < Cow < 'a , str > > for String { fn extend < I : IntoIterator < Item = Cow < 'a , str > > > (& mut self , iter : I) { iter . into_iter () . for_each (move | s | self . push_str (& s)) ; } # [inline] fn extend_one (& mut self , s : Cow < 'a , str >) { self . push_str (& s) ; } }
};
}
