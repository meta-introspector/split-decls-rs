// Generated macro for tests (module)
macro_rules! Depcrate_arch_all_shiftortests {
() => {
// Module: crate::arch::all::shiftor
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; define_substring_forward_quickcheck ! (| h , n | Some (Finder :: new (n) ?. find (h))) ; # [test] fn forward () { crate :: tests :: substring :: Runner :: new () . fwd (| h , n | Some (Finder :: new (n) ? . find (h))) . run () ; } }
};
}
