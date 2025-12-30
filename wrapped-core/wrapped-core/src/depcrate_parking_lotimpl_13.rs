// Generated macro for impl_13 (impl)
macro_rules! Depcrate_parking_lotimpl_13 {
() => {
// Module: crate::parking_lot
// Provides: {"impl_13"}
// Dependencies: {}
impl HashTable { # [inline] fn new (num_threads : usize , prev : * const HashTable) -> Box < HashTable > { let new_size = (num_threads * LOAD_FACTOR) . next_power_of_two () ; let hash_bits = 0usize . leading_zeros () - new_size . leading_zeros () - 1 ; let now = TimeoutInstant :: now () ; let mut entries = Vec :: with_capacity (new_size) ; for i in 0 .. new_size { entries . push (Bucket :: new (now , i as u32 + 1)) ; } Box :: new (HashTable { entries : entries . into_boxed_slice () , hash_bits , _prev : prev , }) } }
};
}
