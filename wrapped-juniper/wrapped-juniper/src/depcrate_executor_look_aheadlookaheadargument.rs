// Generated macro for LookAheadArgument (struct)
macro_rules! Depcrate_executor_look_aheadLookAheadArgument {
() => {
// Module: crate::executor::look_ahead
// Provides: {"LookAheadArgument"}
// Dependencies: {}
# [doc = " [Lazy][2]-evaluated [argument] used in [look-ahead][0] operations on an executed GraphQL query."] # [doc = ""] # [doc = " [0]: https://en.wikipedia.org/wiki/Look-ahead_(backtracking)"] # [doc = " [2]: https://en.wikipedia.org/wiki/Lazy_evaluation"] # [doc = " [argument]: https://spec.graphql.org/October2021#sec-Language.Arguments"] # [derive (Debug)] # [must_use] pub struct LookAheadArgument < 'a , S > { name : & 'a Spanning < & 'a str > , input_value : & 'a Spanning < InputValue < S > > , vars : & 'a Variables < S > , }
};
}
