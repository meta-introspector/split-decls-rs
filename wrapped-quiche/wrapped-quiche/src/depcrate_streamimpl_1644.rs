// Generated macro for impl_1644 (impl)
macro_rules! Depcrate_streamimpl_1644 {
() => {
// Module: crate::stream
// Provides: {"impl_1644"}
// Dependencies: {}
impl PartialOrd for StreamPriorityKey { # [allow (clippy :: non_canonical_partial_ord_impl)] fn partial_cmp (& self , other : & Self) -> Option < cmp :: Ordering > { if self . id == other . id { return Some (cmp :: Ordering :: Equal) ; } if self . urgency != other . urgency { return self . urgency . partial_cmp (& other . urgency) ; } if ! self . incremental && ! other . incremental { return self . id . partial_cmp (& other . id) ; } if self . incremental && ! other . incremental { return Some (cmp :: Ordering :: Greater) ; } if ! self . incremental && other . incremental { return Some (cmp :: Ordering :: Less) ; } Some (cmp :: Ordering :: Greater) } }
};
}
