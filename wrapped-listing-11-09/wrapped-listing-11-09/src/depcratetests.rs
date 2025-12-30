// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] # [should_panic (expected = "less than or equal to 100")] fn greater_than_100 () { Guess :: new (200) ; } }
};
}
