// Generated macro for DecodeEstimate (trait)
macro_rules! Depcrate_engineDecodeEstimate {
() => {
// Module: crate::engine
// Provides: {"DecodeEstimate"}
// Dependencies: {}
# [doc = " The decode estimate used by an engine implementation. Users do not need to interact with this;"] # [doc = " it is only for engine implementors."] # [doc = ""] # [doc = " Implementors may store relevant data here when constructing this to avoid having to calculate"] # [doc = " them again during actual decoding."] pub trait DecodeEstimate { # [doc = " Returns a conservative (err on the side of too big) estimate of the decoded length to use"] # [doc = " for pre-allocating buffers, etc."] # [doc = ""] # [doc = " The estimate must be no larger than the next largest complete triple of decoded bytes."] # [doc = " That is, the final quad of tokens to decode may be assumed to be complete with no padding."] fn decoded_len_estimate (& self) -> usize ; }
};
}
