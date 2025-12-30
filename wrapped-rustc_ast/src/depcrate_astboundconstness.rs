// Generated macro for BoundConstness (enum)
macro_rules! Depcrate_astBoundConstness {
() => {
// Module: crate::ast
// Provides: {"BoundConstness"}
// Dependencies: {}
# [doc = " The constness of a trait bound."] # [derive (Copy , Clone , PartialEq , Eq , Encodable , Decodable , Debug , Hash)] # [derive (HashStable_Generic , Walkable)] pub enum BoundConstness { # [doc = " `Type: Trait`"] Never , # [doc = " `Type: const Trait`"] Always (Span) , # [doc = " `Type: [const] Trait`"] Maybe (Span) , }
};
}
