// Generated macro for impl_457 (impl)
macro_rules! Depcrate_traitsimpl_457 {
() => {
// Module: crate::traits
// Provides: {"impl_457"}
// Dependencies: {}
# [cfg (feature = "std")] impl HexDisplay for str { # [allow (unused_variables)] fn to_hex (& self , chunk_size : usize) -> String { self . to_hex_from (chunk_size , 0) } # [allow (unused_variables)] fn to_hex_from (& self , chunk_size : usize , from : usize) -> String { self . as_bytes () . to_hex_from (chunk_size , from) } }
};
}
