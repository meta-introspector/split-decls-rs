// Generated macro for impl_234 (impl)
macro_rules! Depcrate_item_treeimpl_234 {
() => {
// Module: crate::item_tree
// Provides: {"impl_234"}
// Dependencies: {}
impl Index < RawVisibilityId > for ItemTree { type Output = RawVisibility ; fn index (& self , index : RawVisibilityId) -> & Self :: Output { static VIS_PUB : RawVisibility = RawVisibility :: Public ; static VIS_PRIV_IMPLICIT : RawVisibility = RawVisibility :: PubSelf (VisibilityExplicitness :: Implicit) ; static VIS_PRIV_EXPLICIT : RawVisibility = RawVisibility :: PubSelf (VisibilityExplicitness :: Explicit) ; static VIS_PUB_CRATE : RawVisibility = RawVisibility :: PubCrate ; match index { RawVisibilityId :: PRIV_IMPLICIT => & VIS_PRIV_IMPLICIT , RawVisibilityId :: PRIV_EXPLICIT => & VIS_PRIV_EXPLICIT , RawVisibilityId :: PUB => & VIS_PUB , RawVisibilityId :: PUB_CRATE => & VIS_PUB_CRATE , _ => & self . vis . arena [index . 0 as usize] , } } }
};
}
