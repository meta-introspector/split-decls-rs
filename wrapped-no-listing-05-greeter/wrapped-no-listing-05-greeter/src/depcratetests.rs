// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn greeting_contains_name () { let result = greeting ("Carol") ; assert ! (result . contains ("Carol")) ; } }
};
}
