// Generated macro for PartialConstStability (struct)
macro_rules! Depcrate_stabilityPartialConstStability {
() => {
// Module: crate::stability
// Provides: {"PartialConstStability"}
// Dependencies: {}
# [doc = " Excludes `const_stable_indirect`. This is necessary because when `-Zforce-unstable-if-unmarked`"] # [doc = " is set, we need to encode standalone `#[rustc_const_stable_indirect]` attributes"] # [derive (Encodable , Decodable , Copy , Clone , Debug , PartialEq , Eq , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub struct PartialConstStability { pub level : StabilityLevel , pub feature : Symbol , # [doc = " whether the function has a `#[rustc_promotable]` attribute"] pub promotable : bool , }
};
}
