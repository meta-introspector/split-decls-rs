// Generated macro for impl_743 (impl)
macro_rules! Depcrate_collectionimpl_743 {
() => {
// Module: crate::collection
// Provides: {"impl_743"}
// Dependencies: {}
impl < T : Strategy > Strategy for VecStrategy < T > { type Tree = VecValueTree < T :: Tree > ; type Value = Vec < T :: Value > ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { let (start , end) = self . size . start_end_incl () ; let max_size = sample_uniform_incl (runner , start , end) ; let mut elements = Vec :: with_capacity (max_size) ; while elements . len () < max_size { elements . push (self . element . new_tree (runner) ?) ; } Ok (VecValueTree { elements , included_elements : VarBitSet :: saturated (max_size) , min_size : start , shrink : Shrink :: DeleteElement (0) , prev_shrink : None , }) } }
};
}
