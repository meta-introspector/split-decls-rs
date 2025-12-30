// Generated macro for StabilityLevel (enum)
macro_rules! Depcrate_stabilityStabilityLevel {
() => {
// Module: crate::stability
// Provides: {"StabilityLevel"}
// Dependencies: {}
# [doc = " The available stability levels."] # [derive (Encodable , Decodable , PartialEq , Copy , Clone , Debug , Eq , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub enum StabilityLevel { # [doc = " `#[unstable]`"] Unstable { # [doc = " Reason for the current stability level."] reason : UnstableReason , # [doc = " Relevant `rust-lang/rust` issue."] issue : Option < NonZero < u32 > > , is_soft : bool , # [doc = " If part of a feature is stabilized and a new feature is added for the remaining parts,"] # [doc = " then the `implied_by` attribute is used to indicate which now-stable feature previously"] # [doc = " contained an item."] # [doc = ""] # [doc = " ```pseudo-Rust"] # [doc = " #[unstable(feature = \"foo\", issue = \"...\")]"] # [doc = " fn foo() {}"] # [doc = " #[unstable(feature = \"foo\", issue = \"...\")]"] # [doc = " fn foobar() {}"] # [doc = " ```"] # [doc = ""] # [doc = " ...becomes..."] # [doc = ""] # [doc = " ```pseudo-Rust"] # [doc = " #[stable(feature = \"foo\", since = \"1.XX.X\")]"] # [doc = " fn foo() {}"] # [doc = " #[unstable(feature = \"foobar\", issue = \"...\", implied_by = \"foo\")]"] # [doc = " fn foobar() {}"] # [doc = " ```"] implied_by : Option < Symbol > , old_name : Option < Symbol > , } , # [doc = " `#[stable]`"] Stable { # [doc = " Rust release which stabilized this feature."] since : StableSince , # [doc = " This is `Some` if this item allowed to be referred to on stable via unstable modules;"] # [doc = " the `Symbol` is the deprecation message printed in that case."] allowed_through_unstable_modules : Option < Symbol > , } , }
};
}
