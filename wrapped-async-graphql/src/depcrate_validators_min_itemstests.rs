// Generated macro for tests (module)
macro_rules! Depcrate_validators_min_itemstests {
() => {
// Module: crate::validators::min_items
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_min_items () { assert ! (min_items (& vec ! [1 , 2] , 3) . is_err ()) ; assert ! (min_items (& vec ! [1 , 2 , 3] , 3) . is_ok ()) ; assert ! (min_items (& vec ! [1 , 2 , 3 , 4] , 3) . is_ok ()) ; } }
};
}
