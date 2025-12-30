// Generated macro for Stability (struct)
macro_rules! Depcrate_stabilityStability {
() => {
// Module: crate::stability
// Provides: {"Stability"}
// Dependencies: {}
# [doc = " Represents the following attributes:"] # [doc = ""] # [doc = " - `#[stable]`"] # [doc = " - `#[unstable]`"] # [derive (Encodable , Decodable , Copy , Clone , Debug , PartialEq , Eq , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub struct Stability { pub level : StabilityLevel , pub feature : Symbol , }
};
}
