// Generated macro for BoundAsyncness (enum)
macro_rules! Depcrate_astBoundAsyncness {
() => {
// Module: crate::ast
// Provides: {"BoundAsyncness"}
// Dependencies: {}
# [doc = " The asyncness of a trait bound."] # [derive (Copy , Clone , PartialEq , Eq , Encodable , Decodable , Debug)] # [derive (HashStable_Generic , Walkable)] pub enum BoundAsyncness { # [doc = " `Type: Trait`"] Normal , # [doc = " `Type: async Trait`"] Async (Span) , }
};
}
