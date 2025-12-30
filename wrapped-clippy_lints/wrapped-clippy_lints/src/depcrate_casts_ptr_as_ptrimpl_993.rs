// Generated macro for impl_993 (impl)
macro_rules! Depcrate_casts_ptr_as_ptrimpl_993 {
() => {
// Module: crate::casts::ptr_as_ptr
// Provides: {"impl_993"}
// Dependencies: {}
impl OmitFollowedCastReason < '_ > { fn corresponding_item (& self) -> Option < & QPath < '_ > > { match self { OmitFollowedCastReason :: None => None , OmitFollowedCastReason :: Null (x) | OmitFollowedCastReason :: NullMut (x) => Some (* x) , } } }
};
}
