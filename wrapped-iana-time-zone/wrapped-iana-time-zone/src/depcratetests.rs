// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn get_current () { println ! ("current: {}" , get_timezone () . unwrap ()) ; } }
};
}
