// Generated macro for impl_95 (impl)
macro_rules! Depcrate_ordered_setimpl_95 {
() => {
// Module: crate::ordered_set
// Provides: {"impl_95"}
// Dependencies: {}
impl < T > OrderedSet < T > where T : Eq + PhfHash + PhfBorrow < T > , { # [doc = " Returns true if `other` shares no elements with `self`."] # [inline] pub fn is_disjoint (& self , other : & OrderedSet < T >) -> bool { ! self . iter () . any (| value | other . contains (value)) } # [doc = " Returns true if `other` contains all values in `self`."] # [inline] pub fn is_subset (& self , other : & OrderedSet < T >) -> bool { self . iter () . all (| value | other . contains (value)) } # [doc = " Returns true if `self` contains all values in `other`."] # [inline] pub fn is_superset (& self , other : & OrderedSet < T >) -> bool { other . is_subset (self) } }
};
}
