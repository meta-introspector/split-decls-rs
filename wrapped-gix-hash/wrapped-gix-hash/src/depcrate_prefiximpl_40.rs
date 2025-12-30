// Generated macro for impl_40 (impl)
macro_rules! Depcrate_prefiximpl_40 {
() => {
// Module: crate::prefix
// Provides: {"impl_40"}
// Dependencies: {}
impl std :: fmt :: Display for Prefix { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . bytes . to_hex_with_len (self . hex_len) . fmt (f) } }
};
}
