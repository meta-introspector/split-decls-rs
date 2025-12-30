// Generated macro for tests (module)
macro_rules! Depcrate_arch_all_rabinkarptests {
() => {
// Module: crate::arch::all::rabinkarp
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; define_substring_forward_quickcheck ! (| h , n | Some (Finder :: new (n) . find (h , n))) ; define_substring_reverse_quickcheck ! (| h , n | Some (FinderRev :: new (n) . rfind (h , n))) ; # [test] fn forward () { crate :: tests :: substring :: Runner :: new () . fwd (| h , n | Some (Finder :: new (n) . find (h , n))) . run () ; } # [test] fn reverse () { crate :: tests :: substring :: Runner :: new () . rev (| h , n | Some (FinderRev :: new (n) . rfind (h , n))) . run () ; } }
};
}
