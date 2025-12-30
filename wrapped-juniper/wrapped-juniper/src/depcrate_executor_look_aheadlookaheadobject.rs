// Generated macro for LookAheadObject (struct)
macro_rules! Depcrate_executor_look_aheadLookAheadObject {
() => {
// Module: crate::executor::look_ahead
// Provides: {"LookAheadObject"}
// Dependencies: {}
# [doc = " [Lazy][2]-evaluated [input object] used in [look-ahead][0] operations on an executed GraphQL"] # [doc = " query."] # [doc = ""] # [doc = " [0]: https://en.wikipedia.org/wiki/Look-ahead_(backtracking)"] # [doc = " [2]: https://en.wikipedia.org/wiki/Lazy_evaluation"] # [doc = " [input object]: https://spec.graphql.org/October2021#sec-Input-Objects"] # [derive (Debug)] # [must_use] pub struct LookAheadObject < 'a , S > { input_object : & 'a [(Spanning < String > , Spanning < InputValue < S > >)] , vars : Option < & 'a Variables < S > > , }
};
}
