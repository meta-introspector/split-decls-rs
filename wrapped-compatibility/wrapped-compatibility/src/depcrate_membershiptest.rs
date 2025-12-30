// Generated macro for test (function)
macro_rules! Depcrate_membershiptest {
() => {
// Module: crate::membership
// Provides: {"test"}
// Dependencies: {}
# [test] pub fn test () { let mut rng = rand :: thread_rng () ; for _ in 0 .. 1000 { crate :: test_same (Membership { learners : random_btreeset (& mut rng) , configs : vec_random_btreeset (& mut rng) , all_members : random_btreeset (& mut rng) , }) ; } }
};
}
