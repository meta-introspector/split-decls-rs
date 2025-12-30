// Generated macro for impl_145 (impl)
macro_rules! Depcrate_setimpl_145 {
() => {
// Module: crate::set
// Provides: {"impl_145"}
// Dependencies: {}
impl < T , S1 , S2 > BitXor < & IndexSet < T , S2 > > for & IndexSet < T , S1 > where T : Eq + Hash + Clone , S1 : BuildHasher + Default , S2 : BuildHasher , { type Output = IndexSet < T , S1 > ; # [doc = " Returns the set symmetric-difference, cloned into a new set."] # [doc = ""] # [doc = " Values from `self` are collected in their original order, followed by"] # [doc = " values from `other` in their original order."] fn bitxor (self , other : & IndexSet < T , S2 >) -> Self :: Output { self . symmetric_difference (other) . cloned () . collect () } }
};
}
