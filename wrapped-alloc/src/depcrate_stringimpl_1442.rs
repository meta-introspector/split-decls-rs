// Generated macro for impl_1442 (impl)
macro_rules! Depcrate_stringimpl_1442 {
() => {
// Module: crate::string
// Provides: {"impl_1442"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "extend_ref" , since = "1.2.0")] impl < 'a > Extend < & 'a char > for String { fn extend < I : IntoIterator < Item = & 'a char > > (& mut self , iter : I) { self . extend (iter . into_iter () . cloned ()) ; } # [inline] fn extend_one (& mut self , & c : & 'a char) { self . push (c) ; } # [inline] fn extend_reserve (& mut self , additional : usize) { self . reserve (additional) ; } }
};
}
