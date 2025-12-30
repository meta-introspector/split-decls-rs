// Generated macro for impl_115 (impl)
macro_rules! Depcrate_setimpl_115 {
() => {
// Module: crate::set
// Provides: {"impl_115"}
// Dependencies: {}
impl < T > Set < T > where T : Eq + PhfHash + PhfBorrow < T > , { # [doc = " Returns true if `other` shares no elements with `self`."] pub fn is_disjoint (& self , other : & Set < T >) -> bool { ! self . iter () . any (| value | other . contains (value)) } # [doc = " Returns true if `other` contains all values in `self`."] pub fn is_subset (& self , other : & Set < T >) -> bool { self . iter () . all (| value | other . contains (value)) } # [doc = " Returns true if `self` contains all values in `other`."] pub fn is_superset (& self , other : & Set < T >) -> bool { other . is_subset (self) } }
};
}
