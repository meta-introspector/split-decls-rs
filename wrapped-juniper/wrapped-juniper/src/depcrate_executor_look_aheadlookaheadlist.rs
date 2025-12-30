// Generated macro for LookAheadList (struct)
macro_rules! Depcrate_executor_look_aheadLookAheadList {
() => {
// Module: crate::executor::look_ahead
// Provides: {"LookAheadList"}
// Dependencies: {}
# [doc = " [Lazy][2]-evaluated [list] used in [look-ahead][0] operations on an executed GraphQL query."] # [doc = ""] # [doc = " [0]: https://en.wikipedia.org/wiki/Look-ahead_(backtracking)"] # [doc = " [2]: https://en.wikipedia.org/wiki/Lazy_evaluation"] # [doc = " [list]: https://spec.graphql.org/October2021#sec-List"] # [derive (Debug)] # [must_use] pub struct LookAheadList < 'a , S > { input_list : & 'a [Spanning < InputValue < S > >] , vars : Option < & 'a Variables < S > > , }
};
}
