// Generated macro for impl_143 (impl)
macro_rules! Depcrate_setimpl_143 {
() => {
// Module: crate::set
// Provides: {"impl_143"}
// Dependencies: {}
impl < T , S1 , S2 > BitAnd < & IndexSet < T , S2 > > for & IndexSet < T , S1 > where T : Eq + Hash + Clone , S1 : BuildHasher + Default , S2 : BuildHasher , { type Output = IndexSet < T , S1 > ; # [doc = " Returns the set intersection, cloned into a new set."] # [doc = ""] # [doc = " Values are collected in the same order that they appear in `self`."] fn bitand (self , other : & IndexSet < T , S2 >) -> Self :: Output { self . intersection (other) . cloned () . collect () } }
};
}
