// Generated macro for Defaultness (enum)
macro_rules! Depcrate_astDefaultness {
() => {
// Module: crate::ast
// Provides: {"Defaultness"}
// Dependencies: {}
# [doc = " Item defaultness."] # [doc = " For details see the [RFC #2532](https://github.com/rust-lang/rfcs/pull/2532)."] # [derive (Copy , Clone , PartialEq , Encodable , Decodable , Debug , HashStable_Generic , Walkable)] pub enum Defaultness { Default (Span) , Final , }
};
}
