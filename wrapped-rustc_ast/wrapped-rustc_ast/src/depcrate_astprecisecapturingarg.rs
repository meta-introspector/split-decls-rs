// Generated macro for PreciseCapturingArg (enum)
macro_rules! Depcrate_astPreciseCapturingArg {
() => {
// Module: crate::ast
// Provides: {"PreciseCapturingArg"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum PreciseCapturingArg { # [doc = " Lifetime parameter."] Lifetime (# [visitable (extra = LifetimeCtxt :: GenericArg)] Lifetime) , # [doc = " Type or const parameter."] Arg (Path , NodeId) , }
};
}
