// Generated macro for SummarySink (struct)
macro_rules! Depcrate_summarySummarySink {
() => {
// Module: crate::summary
// Provides: {"SummarySink"}
// Dependencies: {}
# [doc = " An implementation of `Sink` associated with a matcher and an optional file"] # [doc = " path for the summary printer."] # [doc = ""] # [doc = " This type is generic over a few type parameters:"] # [doc = ""] # [doc = " * `'p` refers to the lifetime of the file path, if one is provided. When"] # [doc = " no file path is given, then this is `'static`."] # [doc = " * `'s` refers to the lifetime of the [`Summary`] printer that this type"] # [doc = " borrows."] # [doc = " * `M` refers to the type of matcher used by"] # [doc = " `grep_searcher::Searcher` that is reporting results to this sink."] # [doc = " * `W` refers to the underlying writer that this printer is writing its"] # [doc = " output to."] # [derive (Debug)] pub struct SummarySink < 'p , 's , M : Matcher , W > { matcher : M , summary : & 's mut Summary < W > , interpolator : hyperlink :: Interpolator , path : Option < PrinterPath < 'p > > , start_time : Instant , match_count : u64 , binary_byte_offset : Option < u64 > , stats : Option < Stats > , }
};
}
