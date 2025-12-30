// Generated macro for impl_536 (impl)
macro_rules! Depcrate_collections_btree_navigateimpl_536 {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"impl_536"}
// Dependencies: {}
impl < BorrowType : marker :: BorrowType , K , V > LeafRange < BorrowType , K , V > { # [doc = " If possible, extract some result from the following KV and move to the edge beyond it."] fn perform_next_checked < F , R > (& mut self , f : F) -> Option < R > where F : Fn (& Handle < NodeRef < BorrowType , K , V , marker :: LeafOrInternal > , marker :: KV >) -> R , { if self . is_empty () { None } else { super :: mem :: replace (self . front . as_mut () . unwrap () , | front | { let kv = front . next_kv () . ok () . unwrap () ; let result = f (& kv) ; (kv . next_leaf_edge () , Some (result)) }) } } # [doc = " If possible, extract some result from the preceding KV and move to the edge beyond it."] fn perform_next_back_checked < F , R > (& mut self , f : F) -> Option < R > where F : Fn (& Handle < NodeRef < BorrowType , K , V , marker :: LeafOrInternal > , marker :: KV >) -> R , { if self . is_empty () { None } else { super :: mem :: replace (self . back . as_mut () . unwrap () , | back | { let kv = back . next_back_kv () . ok () . unwrap () ; let result = f (& kv) ; (kv . next_back_leaf_edge () , Some (result)) }) } } }
};
}
