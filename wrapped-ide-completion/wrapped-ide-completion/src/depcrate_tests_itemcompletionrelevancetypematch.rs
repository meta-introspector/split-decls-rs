// Generated macro for CompletionRelevanceTypeMatch (enum)
macro_rules! Depcrate_tests_itemCompletionRelevanceTypeMatch {
() => {
// Module: crate::tests::item
// Provides: {"CompletionRelevanceTypeMatch"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum CompletionRelevanceTypeMatch { # [doc = " This is set in cases like these:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " enum Option<T> { Some(T), None }"] # [doc = " fn f(a: Option<u32>) {}"] # [doc = " fn main {"] # [doc = "     f(Option::N$0) // type `Option<T>` could unify with `Option<u32>`"] # [doc = " }"] # [doc = " ```"] CouldUnify , # [doc = " This is set in cases where the type matches the expected type, like:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " fn f(spam: String) {}"] # [doc = " fn main() {"] # [doc = "     let foo = String::new();"] # [doc = "     f($0) // type of local matches the type of param"] # [doc = " }"] # [doc = " ```"] Exact , }
};
}
