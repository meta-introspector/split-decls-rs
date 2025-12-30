// Generated macro for impl_161 (impl)
macro_rules! Depcrateimpl_161 {
() => {
// Module: crate
// Provides: {"impl_161"}
// Dependencies: {}
# [cfg (feature = "kv")] impl < 'a > fmt :: Debug for KeyValues < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut visitor = f . debug_map () ; self . 0 . visit (& mut visitor) . map_err (| _ | fmt :: Error) ? ; visitor . finish () } }
};
}
