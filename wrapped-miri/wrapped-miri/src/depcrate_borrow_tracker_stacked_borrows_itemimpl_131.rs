// Generated macro for impl_131 (impl)
macro_rules! Depcrate_borrow_tracker_stacked_borrows_itemimpl_131 {
() => {
// Module: crate::borrow_tracker::stacked_borrows::item
// Provides: {"impl_131"}
// Dependencies: {}
impl Permission { const UNIQUE : u64 = 0 ; const SHARED_READ_WRITE : u64 = 1 ; const SHARED_READ_ONLY : u64 = 2 ; const DISABLED : u64 = 3 ; fn to_bits (self) -> u64 { match self { Permission :: Unique => Self :: UNIQUE , Permission :: SharedReadWrite => Self :: SHARED_READ_WRITE , Permission :: SharedReadOnly => Self :: SHARED_READ_ONLY , Permission :: Disabled => Self :: DISABLED , } } fn from_bits (perm : u64) -> Self { match perm { Self :: UNIQUE => Permission :: Unique , Self :: SHARED_READ_WRITE => Permission :: SharedReadWrite , Self :: SHARED_READ_ONLY => Permission :: SharedReadOnly , Self :: DISABLED => Permission :: Disabled , _ => unreachable ! () , } } }
};
}
