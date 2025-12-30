// Generated macro for impl_140 (impl)
macro_rules! Depcrate_setimpl_140 {
() => {
// Module: crate::set
// Provides: {"impl_140"}
// Dependencies: {}
impl < T , S1 , S2 > PartialEq < IndexSet < T , S2 > > for IndexSet < T , S1 > where T : Hash + Eq , S1 : BuildHasher , S2 : BuildHasher , { fn eq (& self , other : & IndexSet < T , S2 >) -> bool { self . len () == other . len () && self . is_subset (other) } }
};
}
