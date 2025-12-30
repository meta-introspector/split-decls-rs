// Generated macro for impl_220 (impl)
macro_rules! Depcrate_item_treeimpl_220 {
() => {
// Module: crate::item_tree
// Provides: {"impl_220"}
// Dependencies: {}
impl fmt :: Debug for RawVisibilityId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut f = f . debug_tuple ("RawVisibilityId") ; match * self { Self :: PUB => f . field (& "pub") , Self :: PRIV_IMPLICIT | Self :: PRIV_EXPLICIT => f . field (& "pub(self)") , Self :: PUB_CRATE => f . field (& "pub(crate)") , _ => f . field (& self . 0) , } ; f . finish () } }
};
}
