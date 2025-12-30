// Generated macro for impl_106 (impl)
macro_rules! Depcrate_paserkimpl_106 {
() => {
// Module: crate::paserk
// Provides: {"impl_106"}
// Dependencies: {}
# [cfg (feature = "v3")] impl FormatAsPaserk for AsymmetricPublicKey < V3 > { fn fmt (& self , write : & mut dyn Write) -> core :: fmt :: Result { write . write_str ("k3.public.") ? ; write . write_str (& encode_b64 (self . as_bytes ()) . map_err (| _ | core :: fmt :: Error) ?) } }
};
}
