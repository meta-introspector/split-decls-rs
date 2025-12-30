// Generated macro for leading_space (function)
macro_rules! Depcrate_snapshotleading_space {
() => {
// Module: crate::snapshot
// Provides: {"leading_space"}
// Dependencies: {}
fn leading_space (value : & str) -> String { value . chars () . take_while (| x | x . is_whitespace ()) . collect :: < String > () }
};
}
