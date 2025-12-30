// Generated macro for allocating (module)
macro_rules! Depcrate_referencedallocating {
() => {
// Module: crate::referenced
// Provides: {"allocating"}
// Dependencies: {}
# [cfg (feature = "alloc")] mod allocating { use super :: { OwnedToRef , RefToOwned } ; use alloc :: boxed :: Box ; impl < 'a > RefToOwned < 'a > for & 'a [u8] { type Owned = Box < [u8] > ; fn ref_to_owned (& self) -> Self :: Owned { Box :: from (* self) } } impl OwnedToRef for Box < [u8] > { type Borrowed < 'a > = & 'a [u8] ; fn owned_to_ref (& self) -> Self :: Borrowed < '_ > { self . as_ref () } } }
};
}
