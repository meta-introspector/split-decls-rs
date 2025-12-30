// Generated macro for impl_108 (impl)
macro_rules! Depcrate_paserkimpl_108 {
() => {
// Module: crate::paserk
// Provides: {"impl_108"}
// Dependencies: {}
# [cfg (feature = "v4")] impl FormatAsPaserk for AsymmetricPublicKey < V4 > { fn fmt (& self , write : & mut dyn Write) -> core :: fmt :: Result { write . write_str ("k4.public.") ? ; write . write_str (& encode_b64 (self . as_bytes ()) . map_err (| _ | core :: fmt :: Error) ?) } }
};
}
