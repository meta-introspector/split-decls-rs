// Generated macro for TraitBoundModifiers (struct)
macro_rules! Depcrate_astTraitBoundModifiers {
() => {
// Module: crate::ast
// Provides: {"TraitBoundModifiers"}
// Dependencies: {}
# [doc = " Modifiers on a trait bound like `[const]`, `?` and `!`."] # [derive (Copy , Clone , PartialEq , Eq , Encodable , Decodable , Debug , Walkable)] pub struct TraitBoundModifiers { pub constness : BoundConstness , pub asyncness : BoundAsyncness , pub polarity : BoundPolarity , }
};
}
