// Generated macro for tests (module)
macro_rules! Depcrate_validators_max_itemstests {
() => {
// Module: crate::validators::max_items
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_max_items () { assert ! (max_items (& vec ! [1 , 2] , 3) . is_ok ()) ; assert ! (max_items (& vec ! [1 , 2 , 3] , 3) . is_ok ()) ; assert ! (max_items (& vec ! [1 , 2 , 3 , 4] , 3) . is_err ()) ; } }
};
}
