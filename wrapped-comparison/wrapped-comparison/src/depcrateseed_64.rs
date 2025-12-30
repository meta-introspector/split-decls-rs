// Generated macro for seed_64 (function)
macro_rules! Depcrateseed_64 {
() => {
// Module: crate
// Provides: {"seed_64"}
// Dependencies: {}
fn seed_64 () -> impl Strategy < Value = u64 > { prop_oneof ! [Just (0) , Just (u64 :: MAX) , num :: u64 :: ANY] }
};
}
