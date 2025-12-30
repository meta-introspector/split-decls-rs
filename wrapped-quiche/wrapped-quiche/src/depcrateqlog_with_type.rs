// Generated macro for qlog_with_type (macro)
macro_rules! Depcrateqlog_with_type {
() => {
// Module: crate
// Provides: {"qlog_with_type"}
// Dependencies: {}
# [doc = " Executes the provided body if the qlog feature is enabled, quiche has been"] # [doc = " configured with a log writer, the event's importance is within the"] # [doc = " configured level."] macro_rules ! qlog_with_type { ($ ty : expr , $ qlog : expr , $ qlog_streamer_ref : ident , $ body : block) => { { # [cfg (feature = "qlog")] { if EventImportance :: from ($ ty) . is_contained_in (&$ qlog . level) { if let Some ($ qlog_streamer_ref) = & mut $ qlog . streamer { $ body } } } } } ; }
};
}
