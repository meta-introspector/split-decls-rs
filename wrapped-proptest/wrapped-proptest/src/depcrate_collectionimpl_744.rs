// Generated macro for impl_744 (impl)
macro_rules! Depcrate_collectionimpl_744 {
() => {
// Module: crate::collection
// Provides: {"impl_744"}
// Dependencies: {}
impl < T : Strategy > Strategy for Vec < T > { type Tree = VecValueTree < T :: Tree > ; type Value = Vec < T :: Value > ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { let len = self . len () ; let elements = self . iter () . map (| t | t . new_tree (runner)) . collect :: < Result < Vec < _ > , Reason > > () ? ; Ok (VecValueTree { elements , included_elements : VarBitSet :: saturated (len) , min_size : len , shrink : Shrink :: ShrinkElement (0) , prev_shrink : None , }) } }
};
}
