// Generated macro for tests (module)
macro_rules! Depcrate_actionstests {
() => {
// Module: crate::actions
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: to_index ; # [test] fn test_to_index () { let s = "hello world" ; let idx = to_index (s , 5) ; assert_eq ! (idx , 5) ; let idx = to_index (s , s . len () as u8) ; assert_eq ! (idx , s . len ()) ; let idx = to_index (s , (s . len () + 1) as u8) ; assert_eq ! (idx , 0) ; } }
};
}
