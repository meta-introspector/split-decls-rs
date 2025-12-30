// Generated macro for impl_755 (impl)
macro_rules! Depcrate_spanimpl_755 {
() => {
// Module: crate::span
// Provides: {"impl_755"}
// Dependencies: {}
impl core :: fmt :: Debug for Span { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use crate :: fmt :: StdFmtWrite ; friendly :: DEFAULT_SPAN_PRINTER . print_span (self , StdFmtWrite (f)) . map_err (| _ | core :: fmt :: Error) } }
};
}
