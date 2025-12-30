// Generated macro for impl_157 (impl)
macro_rules! Depcrateimpl_157 {
() => {
// Module: crate
// Provides: {"impl_157"}
// Dependencies: {}
# [cfg (feature = "kv")] impl < 'a > fmt :: Debug for KeyValues < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut visitor = f . debug_map () ; self . 0 . visit (& mut visitor) . map_err (| _ | fmt :: Error) ? ; visitor . finish () } }
};
}
