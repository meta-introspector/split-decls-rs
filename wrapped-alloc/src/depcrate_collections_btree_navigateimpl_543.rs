// Generated macro for impl_543 (impl)
macro_rules! Depcrate_collections_btree_navigateimpl_543 {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"impl_543"}
// Dependencies: {}
impl < BorrowType , K , V > LazyLeafRange < BorrowType , K , V > { pub (super) fn none () -> Self { LazyLeafRange { front : None , back : None } } # [doc = " Temporarily takes out another, immutable equivalent of the same range."] pub (super) fn reborrow (& self) -> LazyLeafRange < marker :: Immut < '_ > , K , V > { LazyLeafRange { front : self . front . as_ref () . map (| f | f . reborrow ()) , back : self . back . as_ref () . map (| b | b . reborrow ()) , } } }
};
}
