// Generated macro for tests (module)
macro_rules! Depcrate_tests_substring_naivetests {
() => {
// Module: crate::tests::substring::naive
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: tests :: substring ; use super :: * ; # [test] fn forward () { substring :: Runner :: new () . fwd (| h , n | Some (find (h , n))) . run () } # [test] fn reverse () { substring :: Runner :: new () . rev (| h , n | Some (rfind (h , n))) . run () } }
};
}
