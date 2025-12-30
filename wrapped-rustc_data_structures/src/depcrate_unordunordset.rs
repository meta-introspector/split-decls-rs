// Generated macro for UnordSet (struct)
macro_rules! Depcrate_unordUnordSet {
() => {
// Module: crate::unord
// Provides: {"UnordSet"}
// Dependencies: {}
# [doc = " This is a set collection type that tries very hard to not expose"] # [doc = " any internal iteration. This is a useful property when trying to"] # [doc = " uphold the determinism invariants imposed by the query system."] # [doc = ""] # [doc = " This collection type is a good choice for set-like collections the"] # [doc = " keys of which don't have a semantic ordering."] # [doc = ""] # [doc = " See [MCP 533](https://github.com/rust-lang/compiler-team/issues/533)"] # [doc = " for more information."] # [derive (Debug , Eq , PartialEq , Clone , Encodable_NoContext , Decodable_NoContext)] pub struct UnordSet < V : Eq + Hash > { inner : FxHashSet < V > , }
};
}
