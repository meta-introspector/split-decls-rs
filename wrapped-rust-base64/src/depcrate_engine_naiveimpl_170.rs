// Generated macro for impl_170 (impl)
macro_rules! Depcrate_engine_naiveimpl_170 {
() => {
// Module: crate::engine::naive
// Provides: {"impl_170"}
// Dependencies: {}
impl NaiveEstimate { fn new (input_len : usize) -> Self { let rem = input_len % Naive :: DECODE_INPUT_CHUNK_SIZE ; let complete_chunk_len = input_len - rem ; Self { rem , complete_chunk_len , } } }
};
}
