// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_is_nonoverlapping () { for dst in 0 .. 8 { assert ! (is_nonoverlapping (10 , 3 , dst , 3)) ; } for dst in 8 .. 13 { assert ! (! is_nonoverlapping (10 , 3 , dst , 3)) ; } for dst in 13 .. 20 { assert ! (is_nonoverlapping (10 , 3 , dst , 3)) ; } assert ! (is_nonoverlapping (usize :: MAX , 3 , usize :: MAX - 1 , 1)) ; assert ! (! is_nonoverlapping (usize :: MAX , 2 , usize :: MAX - 1 , 3)) ; } }
};
}
