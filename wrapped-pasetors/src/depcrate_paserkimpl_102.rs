// Generated macro for impl_102 (impl)
macro_rules! Depcrate_paserkimpl_102 {
() => {
// Module: crate::paserk
// Provides: {"impl_102"}
// Dependencies: {}
# [cfg (feature = "v4")] impl FormatAsPaserk for AsymmetricSecretKey < V4 > { fn fmt (& self , write : & mut dyn Write) -> core :: fmt :: Result { write . write_str ("k4.secret.") ? ; write . write_str (& encode_b64 (self . as_bytes ()) . map_err (| _ | core :: fmt :: Error) ?) } }
};
}
