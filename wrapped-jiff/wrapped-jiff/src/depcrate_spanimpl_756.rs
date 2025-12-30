// Generated macro for impl_756 (impl)
macro_rules! Depcrate_spanimpl_756 {
() => {
// Module: crate::span
// Provides: {"impl_756"}
// Dependencies: {}
impl core :: fmt :: Display for Span { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use crate :: fmt :: StdFmtWrite ; if f . alternate () { friendly :: DEFAULT_SPAN_PRINTER . print_span (self , StdFmtWrite (f)) . map_err (| _ | core :: fmt :: Error) } else { temporal :: DEFAULT_SPAN_PRINTER . print_span (self , StdFmtWrite (f)) . map_err (| _ | core :: fmt :: Error) } } }
};
}
