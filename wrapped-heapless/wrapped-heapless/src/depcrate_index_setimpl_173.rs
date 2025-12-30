// Generated macro for impl_173 (impl)
macro_rules! Depcrate_index_setimpl_173 {
() => {
// Module: crate::index_set
// Provides: {"impl_173"}
// Dependencies: {}
impl < T , S1 , S2 , const N1 : usize , const N2 : usize > PartialEq < IndexSet < T , S2 , N2 > > for IndexSet < T , S1 , N1 > where T : Eq + Hash , S1 : BuildHasher , S2 : BuildHasher , { fn eq (& self , other : & IndexSet < T , S2 , N2 >) -> bool { self . len () == other . len () && self . is_subset (other) } }
};
}
