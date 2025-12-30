// Generated macro for impl_96 (impl)
macro_rules! Depcrate_paserkimpl_96 {
() => {
// Module: crate::paserk
// Provides: {"impl_96"}
// Dependencies: {}
# [cfg (feature = "v4")] impl FormatAsPaserk for SymmetricKey < V4 > { fn fmt (& self , write : & mut dyn Write) -> core :: fmt :: Result { write . write_str ("k4.local.") ? ; write . write_str (& encode_b64 (self . as_bytes ()) . map_err (| _ | core :: fmt :: Error) ?) } }
};
}
