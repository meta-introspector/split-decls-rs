// Generated macro for generate_hash (function)
macro_rules! Depcrategenerate_hash {
() => {
// Module: crate
// Provides: {"generate_hash"}
// Dependencies: {}
pub fn generate_hash < H : PhfHash > (entries : & [H]) -> HashState { generate_hash_with_hash_fn (entries , phf_shared :: hash) }
};
}
