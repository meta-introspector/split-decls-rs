// Generated macro for random_tree1 (function)
macro_rules! Depcrate_scope_testrandom_tree1 {
() => {
// Module: crate::scope::test
// Provides: {"random_tree1"}
// Dependencies: {}
fn random_tree1 (depth : usize , rng : & mut XorShiftRng) -> Tree < u32 > { let children = if depth == 0 { vec ! [] } else { (0 .. rng . random_range (0 .. 4)) . map (| _ | random_tree1 (depth - 1 , rng)) . collect () } ; Tree { value : rng . random_range (0 .. 1_000_000) , children , } }
};
}
