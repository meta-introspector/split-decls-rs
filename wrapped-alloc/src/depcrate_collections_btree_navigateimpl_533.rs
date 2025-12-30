// Generated macro for impl_533 (impl)
macro_rules! Depcrate_collections_btree_navigateimpl_533 {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"impl_533"}
// Dependencies: {}
impl < BorrowType , K , V > LeafRange < BorrowType , K , V > { pub (super) fn none () -> Self { LeafRange { front : None , back : None } } fn is_empty (& self) -> bool { self . front == self . back } # [doc = " Temporarily takes out another, immutable equivalent of the same range."] pub (super) fn reborrow (& self) -> LeafRange < marker :: Immut < '_ > , K , V > { LeafRange { front : self . front . as_ref () . map (| f | f . reborrow ()) , back : self . back . as_ref () . map (| b | b . reborrow ()) , } } }
};
}
