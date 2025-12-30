// Generated macro for impl_1445 (impl)
macro_rules! Depcrate_stringimpl_1445 {
() => {
// Module: crate::string
// Provides: {"impl_1445"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "extend_string" , since = "1.4.0")] impl Extend < String > for String { fn extend < I : IntoIterator < Item = String > > (& mut self , iter : I) { iter . into_iter () . for_each (move | s | self . push_str (& s)) ; } # [inline] fn extend_one (& mut self , s : String) { self . push_str (& s) ; } }
};
}
