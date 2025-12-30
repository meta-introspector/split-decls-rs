// Generated macro for get_deepest_spantrace (function)
macro_rules! Depcrate_handlerget_deepest_spantrace {
() => {
// Module: crate::handler
// Provides: {"get_deepest_spantrace"}
// Dependencies: {}
# [cfg (feature = "capture-spantrace")] pub (crate) fn get_deepest_spantrace < 'a > (error : & 'a (dyn std :: error :: Error + 'static) ,) -> Option < & 'a SpanTrace > { eyre :: Chain :: new (error) . rev () . flat_map (| error | error . span_trace ()) . next () }
};
}
