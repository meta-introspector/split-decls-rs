// Generated macro for path_span_without_args (function)
macro_rules! Depcrate_non_std_lazy_staticspath_span_without_args {
() => {
// Module: crate::non_std_lazy_statics
// Provides: {"path_span_without_args"}
// Dependencies: {}
# [doc = " Return the span of a given `Path` without including any of its args."] # [doc = ""] # [doc = " NB: Re-write of a private function `rustc_lint::non_local_def::path_span_without_args`."] fn path_span_without_args (path : & hir :: Path < '_ >) -> Span { path . segments . last () . and_then (| seg | seg . args) . map_or (path . span , | args | path . span . until (args . span_ext)) }
};
}
