// Generated macro for impl_817 (impl)
macro_rules! Depcrate_collections_linked_listimpl_817 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_817"}
// Dependencies: {}
# [stable (feature = "collection_debug" , since = "1.17.0")] impl < T : fmt :: Debug > fmt :: Debug for Iter < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Iter") . field (& * mem :: ManuallyDrop :: new (LinkedList { head : self . head , tail : self . tail , len : self . len , alloc : Global , marker : PhantomData , })) . field (& self . len) . finish () } }
};
}
