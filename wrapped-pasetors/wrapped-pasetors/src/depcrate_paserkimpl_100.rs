// Generated macro for impl_100 (impl)
macro_rules! Depcrate_paserkimpl_100 {
() => {
// Module: crate::paserk
// Provides: {"impl_100"}
// Dependencies: {}
# [cfg (feature = "v3")] impl FormatAsPaserk for AsymmetricSecretKey < V3 > { fn fmt (& self , write : & mut dyn Write) -> core :: fmt :: Result { write . write_str ("k3.secret.") ? ; write . write_str (& encode_b64 (& self . bytes) . map_err (| _ | core :: fmt :: Error) ?) } }
};
}
