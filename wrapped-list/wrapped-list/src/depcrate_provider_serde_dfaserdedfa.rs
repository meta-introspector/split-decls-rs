// Generated macro for SerdeDFA (struct)
macro_rules! Depcrate_provider_serde_dfaSerdeDFA {
() => {
// Module: crate::provider::serde_dfa
// Provides: {"SerdeDFA"}
// Dependencies: {}
# [doc = " A serde-compatible version of [regex_automata::dfa::sparse::DFA]."] # [doc = ""] # [doc = " This does not implement"] # [doc = " [`serde::Deserialize`] directly, as binary deserialization is not supported in big-endian"] # [doc = " platforms. `Self::maybe_deserialize` can be used to deserialize to `Option<SerdeDFA>`."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Clone , Debug , yoke :: Yokeable , zerofrom :: ZeroFrom)] pub struct SerdeDFA < 'data > { dfa_bytes : VarZeroCow < 'data , [u8] > , # [cfg (feature = "serde_human")] pattern : Option < alloc :: borrow :: Cow < 'data , str > > , }
};
}
