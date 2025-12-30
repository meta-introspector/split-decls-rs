// Generated macro for Selection (enum)
macro_rules! Depcrate_astSelection {
() => {
// Module: crate::ast
// Provides: {"Selection"}
// Dependencies: {}
# [doc = " Entry in a GraphQL selection set"] # [doc = ""] # [doc = " This enum represents one of the three variants of a selection that exists"] # [doc = " in GraphQL: a field, a fragment spread, or an inline fragment. Each of the"] # [doc = " variants references their location in the query source."] # [doc = ""] # [doc = " ```text"] # [doc = " {"] # [doc = "   field(withArg: 123) { subField }"] # [doc = "   ...fragmentSpread"] # [doc = "   ...on User {"] # [doc = "     inlineFragmentField"] # [doc = "   }"] # [doc = " }"] # [doc = " ```"] # [expect (missing_docs , reason = "self-explanatory")] # [derive (Clone , Debug , PartialEq)] pub enum Selection < 'a , S = DefaultScalarValue > { Field (Spanning < Field < 'a , S > >) , FragmentSpread (Spanning < FragmentSpread < 'a , S > >) , InlineFragment (Spanning < InlineFragment < 'a , S > >) , }
};
}
