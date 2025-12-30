// Generated macro for LookAheadSelection (struct)
macro_rules! Depcrate_executor_look_aheadLookAheadSelection {
() => {
// Module: crate::executor::look_ahead
// Provides: {"LookAheadSelection"}
// Dependencies: {}
# [doc = " [Selection] of an executed GraphQL query, used in [look-ahead][0] operations."] # [doc = ""] # [doc = " [0]: https://en.wikipedia.org/wiki/Look-ahead_(backtracking)"] # [doc = " [2]: https://en.wikipedia.org/wiki/Lazy_evaluation"] # [doc = " [Selection]: https://spec.graphql.org/October2021#sec-Selection-Sets"] # [derive (Debug)] # [must_use] pub struct LookAheadSelection < 'a , S > { source : SelectionSource < 'a , S > , applies_for : Applies < 'a > , vars : & 'a Variables < S > , fragments : & 'a HashMap < & 'a str , Fragment < 'a , S > > , }
};
}
