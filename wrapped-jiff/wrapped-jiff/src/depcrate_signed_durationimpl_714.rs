// Generated macro for impl_714 (impl)
macro_rules! Depcrate_signed_durationimpl_714 {
() => {
// Module: crate::signed_duration
// Provides: {"impl_714"}
// Dependencies: {}
impl core :: fmt :: Display for SignedDuration { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use crate :: fmt :: StdFmtWrite ; if f . alternate () { friendly :: DEFAULT_SPAN_PRINTER . print_duration (self , StdFmtWrite (f)) . map_err (| _ | core :: fmt :: Error) } else { temporal :: DEFAULT_SPAN_PRINTER . print_duration (self , StdFmtWrite (f)) . map_err (| _ | core :: fmt :: Error) } } }
};
}
