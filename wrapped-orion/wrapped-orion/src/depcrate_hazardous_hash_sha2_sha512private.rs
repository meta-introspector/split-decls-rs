// Generated macro for private (module)
macro_rules! Depcrate_hazardous_hash_sha2_sha512private {
() => {
// Module: crate::hazardous::hash::sha2::sha512
// Provides: {"private"}
// Dependencies: {}
# [cfg (test)] mod private { use super :: * ; mod test_increment_mlen { use super :: * ; # [test] fn test_mlen_increase_values () { let mut context = Sha512 :: default () ; context . _state . increment_mlen (& WordU64 :: from (1u64)) ; assert_eq ! (context . _state . message_len [0] , WordU64 :: from (0u64)) ; assert_eq ! (context . _state . message_len [1] , WordU64 :: from (8u64)) ; context . _state . increment_mlen (& WordU64 :: from (17u64)) ; assert_eq ! (context . _state . message_len [0] , WordU64 :: from (0u64)) ; assert_eq ! (context . _state . message_len [1] , WordU64 :: from (144u64)) ; context . _state . increment_mlen (& WordU64 :: from (12u64)) ; assert_eq ! (context . _state . message_len [0] , WordU64 :: from (0u64)) ; assert_eq ! (context . _state . message_len [1] , WordU64 :: from (240u64)) ; context . _state . increment_mlen (& WordU64 :: from (u64 :: MAX / 8)) ; assert_eq ! (context . _state . message_len [0] , WordU64 :: from (1u64)) ; assert_eq ! (context . _state . message_len [1] , WordU64 :: from (232u64)) ; } # [test] # [should_panic] fn test_panic_on_second_overflow () { let mut context = Sha512 :: default () ; context . _state . message_len = [WordU64 :: MAX , WordU64 :: from (u64 :: MAX - 7)] ; context . _state . increment_mlen (& WordU64 :: from (1u64)) ; } } }
};
}
