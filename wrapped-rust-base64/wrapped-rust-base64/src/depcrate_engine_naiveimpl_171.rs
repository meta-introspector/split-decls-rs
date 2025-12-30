// Generated macro for impl_171 (impl)
macro_rules! Depcrate_engine_naiveimpl_171 {
() => {
// Module: crate::engine::naive
// Provides: {"impl_171"}
// Dependencies: {}
impl DecodeEstimate for NaiveEstimate { fn decoded_len_estimate (& self) -> usize { ((self . complete_chunk_len / 4) + ((self . rem > 0) as usize)) * 3 } }
};
}
