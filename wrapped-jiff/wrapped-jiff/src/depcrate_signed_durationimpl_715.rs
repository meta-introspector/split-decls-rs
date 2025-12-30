// Generated macro for impl_715 (impl)
macro_rules! Depcrate_signed_durationimpl_715 {
() => {
// Module: crate::signed_duration
// Provides: {"impl_715"}
// Dependencies: {}
impl core :: fmt :: Debug for SignedDuration { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use crate :: fmt :: StdFmtWrite ; if f . alternate () { if self . subsec_nanos () == 0 { write ! (f , "{}s" , self . as_secs ()) } else if self . as_secs () == 0 { write ! (f , "{}ns" , self . subsec_nanos ()) } else { write ! (f , "{}s {}ns" , self . as_secs () , self . subsec_nanos () . unsigned_abs ()) } } else { friendly :: DEFAULT_SPAN_PRINTER . print_duration (self , StdFmtWrite (f)) . map_err (| _ | core :: fmt :: Error) } } }
};
}
