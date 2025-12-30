// Generated macro for impl_104 (impl)
macro_rules! Depcrate_paserkimpl_104 {
() => {
// Module: crate::paserk
// Provides: {"impl_104"}
// Dependencies: {}
# [cfg (feature = "v2")] impl FormatAsPaserk for AsymmetricPublicKey < V2 > { fn fmt (& self , write : & mut dyn Write) -> core :: fmt :: Result { write . write_str ("k2.public.") ? ; write . write_str (& encode_b64 (self . as_bytes ()) . map_err (| _ | core :: fmt :: Error) ?) } }
};
}
