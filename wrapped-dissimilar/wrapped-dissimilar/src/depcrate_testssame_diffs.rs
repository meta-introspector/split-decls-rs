// Generated macro for same_diffs (function)
macro_rules! Depcrate_testssame_diffs {
() => {
// Module: crate::tests
// Provides: {"same_diffs"}
// Dependencies: {}
fn same_diffs (expected : & [Chunk] , actual : & [Diff]) -> bool { fn eq (expected : & str , actual : & Range) -> bool { expected . chars () . eq (slice (* actual) . iter () . copied ()) } expected . len () == actual . len () && expected . iter () . zip (actual) . all (| pair | match pair { (Chunk :: Insert (expected) , Diff :: Insert (actual)) => eq (expected , actual) , (Chunk :: Delete (expected) , Diff :: Delete (actual)) => eq (expected , actual) , (Chunk :: Equal (expected) , Diff :: Equal (actual1 , actual2)) => { eq (expected , actual1) && eq (expected , actual2) } (_ , _) => false , }) }
};
}
