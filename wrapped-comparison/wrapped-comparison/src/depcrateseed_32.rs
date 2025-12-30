// Generated macro for seed_32 (function)
macro_rules! Depcrateseed_32 {
() => {
// Module: crate
// Provides: {"seed_32"}
// Dependencies: {}
fn seed_32 () -> impl Strategy < Value = u32 > { prop_oneof ! [Just (0) , Just (u32 :: MAX) , num :: u32 :: ANY] }
};
}
