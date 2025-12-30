// Generated macro for impl_94 (impl)
macro_rules! Depcrate_paserkimpl_94 {
() => {
// Module: crate::paserk
// Provides: {"impl_94"}
// Dependencies: {}
# [cfg (feature = "v2")] impl FormatAsPaserk for SymmetricKey < V2 > { fn fmt (& self , write : & mut dyn Write) -> core :: fmt :: Result { write . write_str ("k2.local.") ? ; write . write_str (& encode_b64 (self . as_bytes ()) . map_err (| _ | core :: fmt :: Error) ?) } }
};
}
