// Generated macro for impl_354 (impl)
macro_rules! Depcrate_vecimpl_354 {
() => {
// Module: crate::vec
// Provides: {"impl_354"}
// Dependencies: {}
impl < 'a , T : Clone , LenT : LenType , const N : usize > TryFrom < & 'a [T] > for Vec < T , N , LenT > { type Error = CapacityError ; fn try_from (slice : & 'a [T]) -> Result < Self , Self :: Error > { Self :: from_slice (slice) } }
};
}
