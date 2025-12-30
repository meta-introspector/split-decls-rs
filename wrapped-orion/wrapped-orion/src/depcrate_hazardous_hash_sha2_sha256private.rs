// Generated macro for private (module)
macro_rules! Depcrate_hazardous_hash_sha2_sha256private {
() => {
// Module: crate::hazardous::hash::sha2::sha256
// Provides: {"private"}
// Dependencies: {}
# [cfg (test)] mod private { use super :: * ; mod test_increment_mlen { use super :: * ; # [test] fn test_mlen_increase_values () { let mut context = Sha256 :: default () ; context . _state . increment_mlen (& WordU32 :: from (1u32)) ; assert_eq ! (context . _state . message_len [0] , WordU32 :: from (0u32)) ; assert_eq ! (context . _state . message_len [1] , WordU32 :: from (8u32)) ; context . _state . increment_mlen (& WordU32 :: from (17u32)) ; assert_eq ! (context . _state . message_len [0] , WordU32 :: from (0u32)) ; assert_eq ! (context . _state . message_len [1] , WordU32 :: from (144u32)) ; context . _state . increment_mlen (& WordU32 :: from (12u32)) ; assert_eq ! (context . _state . message_len [0] , WordU32 :: from (0u32)) ; assert_eq ! (context . _state . message_len [1] , WordU32 :: from (240u32)) ; context . _state . increment_mlen (& WordU32 :: from (u32 :: MAX / 8)) ; assert_eq ! (context . _state . message_len [0] , WordU32 :: from (1u32)) ; assert_eq ! (context . _state . message_len [1] , WordU32 :: from (232u32)) ; } # [test] # [should_panic] fn test_panic_on_second_overflow () { let mut context = Sha256 :: default () ; context . _state . message_len = [WordU32 :: MAX , WordU32 :: from (u32 :: MAX - 7)] ; context . _state . increment_mlen (& WordU32 :: from (1u32)) ; } } }
};
}
