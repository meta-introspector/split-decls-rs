// Generated macro for ConstStability (struct)
macro_rules! Depcrate_stabilityConstStability {
() => {
// Module: crate::stability
// Provides: {"ConstStability"}
// Dependencies: {}
# [doc = " Represents the `#[rustc_const_unstable]` and `#[rustc_const_stable]` attributes."] # [derive (Encodable , Decodable , Copy , Clone , Debug , PartialEq , Eq , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub struct ConstStability { pub level : StabilityLevel , pub feature : Symbol , # [doc = " whether the function has a `#[rustc_promotable]` attribute"] pub promotable : bool , # [doc = " This is true iff the `const_stable_indirect` attribute is present."] pub const_stable_indirect : bool , }
};
}
