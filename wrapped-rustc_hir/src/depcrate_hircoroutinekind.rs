// Generated macro for CoroutineKind (enum)
macro_rules! Depcrate_hirCoroutineKind {
() => {
// Module: crate::hir
// Provides: {"CoroutineKind"}
// Dependencies: {}
# [doc = " The type of source expression that caused this coroutine to be created."] # [derive (Clone , PartialEq , Eq , Debug , Copy , Hash , HashStable_Generic , Encodable , Decodable)] pub enum CoroutineKind { # [doc = " A coroutine that comes from a desugaring."] Desugared (CoroutineDesugaring , CoroutineSource) , # [doc = " A coroutine literal created via a `yield` inside a closure."] Coroutine (Movability) , }
};
}
