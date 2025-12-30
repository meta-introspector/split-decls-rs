// Generated macro for CoroutineDesugaring (enum)
macro_rules! Depcrate_hirCoroutineDesugaring {
() => {
// Module: crate::hir
// Provides: {"CoroutineDesugaring"}
// Dependencies: {}
# [derive (Clone , PartialEq , Eq , Debug , Copy , Hash , HashStable_Generic , Encodable , Decodable)] pub enum CoroutineDesugaring { # [doc = " An explicit `async` block or the body of an `async` function."] Async , # [doc = " An explicit `gen` block or the body of a `gen` function."] Gen , # [doc = " An explicit `async gen` block or the body of an `async gen` function,"] # [doc = " which is able to both `yield` and `.await`."] AsyncGen , }
};
}
