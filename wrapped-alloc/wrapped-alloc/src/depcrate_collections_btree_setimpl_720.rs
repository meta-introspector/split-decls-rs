// Generated macro for impl_720 (impl)
macro_rules! Depcrate_collections_btree_setimpl_720 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_720"}
// Dependencies: {}
impl < T : Debug , A : Allocator + Clone > Debug for DifferenceInner < '_ , T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { DifferenceInner :: Stitch { self_iter , other_iter } => f . debug_struct ("Stitch") . field ("self_iter" , self_iter) . field ("other_iter" , other_iter) . finish () , DifferenceInner :: Search { self_iter , other_set } => f . debug_struct ("Search") . field ("self_iter" , self_iter) . field ("other_iter" , other_set) . finish () , DifferenceInner :: Iterate (x) => f . debug_tuple ("Iterate") . field (x) . finish () , } } }
};
}
