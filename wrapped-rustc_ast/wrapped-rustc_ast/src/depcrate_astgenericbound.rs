// Generated macro for GenericBound (enum)
macro_rules! Depcrate_astGenericBound {
() => {
// Module: crate::ast
// Provides: {"GenericBound"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum GenericBound { Trait (PolyTraitRef) , Outlives (# [visitable (extra = LifetimeCtxt :: Bound)] Lifetime) , # [doc = " Precise capturing syntax: `impl Sized + use<'a>`"] Use (ThinVec < PreciseCapturingArg > , Span) , }
};
}
