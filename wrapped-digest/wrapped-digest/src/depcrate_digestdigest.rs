// Generated macro for Digest (trait)
macro_rules! Depcrate_digestDigest {
() => {
// Module: crate::digest
// Provides: {"Digest"}
// Dependencies: {}
# [doc = " Convenience wrapper trait covering functionality of cryptographic hash"] # [doc = " functions with fixed output size."] # [doc = ""] # [doc = " This trait wraps [`Update`], [`FixedOutput`], [`Default`], and"] # [doc = " [`HashMarker`] traits and provides additional convenience methods."] pub trait Digest : OutputSizeUser { # [doc = " Create new hasher instance."] fn new () -> Self ; # [doc = " Create new hasher instance which has processed the provided data."] fn new_with_prefix (data : impl AsRef < [u8] >) -> Self ; # [doc = " Process data, updating the internal state."] fn update (& mut self , data : impl AsRef < [u8] >) ; # [doc = " Process input data in a chained manner."] # [must_use] fn chain_update (self , data : impl AsRef < [u8] >) -> Self ; # [doc = " Retrieve result and consume hasher instance."] fn finalize (self) -> Output < Self > ; # [doc = " Write result into provided array and consume the hasher instance."] fn finalize_into (self , out : & mut Output < Self >) ; # [doc = " Retrieve result and reset hasher instance."] fn finalize_reset (& mut self) -> Output < Self > where Self : FixedOutputReset ; # [doc = " Write result into provided array and reset the hasher instance."] fn finalize_into_reset (& mut self , out : & mut Output < Self >) where Self : FixedOutputReset ; # [doc = " Reset hasher instance to its initial state."] fn reset (& mut self) where Self : Reset ; # [doc = " Get output size of the hasher"] fn output_size () -> usize ; # [doc = " Compute hash of `data`."] fn digest (data : impl AsRef < [u8] >) -> Output < Self > ; }
};
}
