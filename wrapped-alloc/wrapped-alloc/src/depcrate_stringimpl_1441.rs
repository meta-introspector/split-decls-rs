// Generated macro for impl_1441 (impl)
macro_rules! Depcrate_stringimpl_1441 {
() => {
// Module: crate::string
// Provides: {"impl_1441"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl Extend < char > for String { fn extend < I : IntoIterator < Item = char > > (& mut self , iter : I) { let iterator = iter . into_iter () ; let (lower_bound , _) = iterator . size_hint () ; self . reserve (lower_bound) ; iterator . for_each (move | c | self . push (c)) ; } # [inline] fn extend_one (& mut self , c : char) { self . push (c) ; } # [inline] fn extend_reserve (& mut self , additional : usize) { self . reserve (additional) ; } }
};
}
