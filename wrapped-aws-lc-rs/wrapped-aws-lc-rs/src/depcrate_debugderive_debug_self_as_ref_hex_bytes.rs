// Generated macro for derive_debug_self_as_ref_hex_bytes (macro)
macro_rules! Depcrate_debugderive_debug_self_as_ref_hex_bytes {
() => {
// Module: crate::debug
// Provides: {"derive_debug_self_as_ref_hex_bytes"}
// Dependencies: {}
# [allow (unused_macros)] macro_rules ! derive_debug_self_as_ref_hex_bytes { ($ typename : ident) => { impl :: core :: fmt :: Debug for $ typename { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> Result < () , :: core :: fmt :: Error > { crate :: debug :: write_hex_tuple (f , stringify ! ($ typename) , self) } } } ; }
};
}
