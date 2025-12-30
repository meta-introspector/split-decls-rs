// Generated macro for ImplPolarity (enum)
macro_rules! Depcrate_astImplPolarity {
() => {
// Module: crate::ast
// Provides: {"ImplPolarity"}
// Dependencies: {}
# [derive (Copy , Clone , PartialEq , Encodable , Decodable , HashStable_Generic , Walkable)] pub enum ImplPolarity { # [doc = " `impl Trait for Type`"] Positive , # [doc = " `impl !Trait for Type`"] Negative (Span) , }
};
}
