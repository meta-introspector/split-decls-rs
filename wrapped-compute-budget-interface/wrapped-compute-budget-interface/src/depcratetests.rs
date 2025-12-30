// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_to_instruction () { let ix = ComputeBudgetInstruction :: set_compute_unit_limit (257) ; assert_eq ! (ix . data , vec ! [2 , 1 , 1 , 0 , 0]) ; let ix = ComputeBudgetInstruction :: set_compute_unit_price (u64 :: MAX) ; assert_eq ! (ix . data , vec ! [3 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255]) ; } }
};
}
