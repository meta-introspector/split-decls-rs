// Generated macro for impl_766 (impl)
macro_rules! Depcrate_collections_btree_setimpl_766 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_766"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator + Clone > Clone for Difference < '_ , T , A > { fn clone (& self) -> Self { Difference { inner : match & self . inner { DifferenceInner :: Stitch { self_iter , other_iter } => DifferenceInner :: Stitch { self_iter : self_iter . clone () , other_iter : other_iter . clone () , } , DifferenceInner :: Search { self_iter , other_set } => { DifferenceInner :: Search { self_iter : self_iter . clone () , other_set } } DifferenceInner :: Iterate (iter) => DifferenceInner :: Iterate (iter . clone ()) , } , } } }
};
}
