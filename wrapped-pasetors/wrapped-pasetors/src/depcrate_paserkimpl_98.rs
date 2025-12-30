// Generated macro for impl_98 (impl)
macro_rules! Depcrate_paserkimpl_98 {
() => {
// Module: crate::paserk
// Provides: {"impl_98"}
// Dependencies: {}
# [cfg (feature = "v2")] impl FormatAsPaserk for AsymmetricSecretKey < V2 > { fn fmt (& self , write : & mut dyn Write) -> core :: fmt :: Result { write . write_str ("k2.secret.") ? ; write . write_str (& encode_b64 (self . as_bytes ()) . map_err (| _ | core :: fmt :: Error) ?) } }
};
}
