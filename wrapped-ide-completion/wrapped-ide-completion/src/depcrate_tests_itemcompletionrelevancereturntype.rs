// Generated macro for CompletionRelevanceReturnType (enum)
macro_rules! Depcrate_tests_itemCompletionRelevanceReturnType {
() => {
// Module: crate::tests::item
// Provides: {"CompletionRelevanceReturnType"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum CompletionRelevanceReturnType { Other , # [doc = " Returns the Self type of the impl/trait"] DirectConstructor , # [doc = " Returns something that indirectly constructs the `Self` type of the impl/trait e.g. `Result<Self, ()>`, `Option<Self>`"] Constructor , # [doc = " Returns a possible builder for the type"] Builder , }
};
}
