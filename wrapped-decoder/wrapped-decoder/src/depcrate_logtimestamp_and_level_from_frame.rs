// Generated macro for timestamp_and_level_from_frame (function)
macro_rules! Depcrate_logtimestamp_and_level_from_frame {
() => {
// Module: crate::log
// Provides: {"timestamp_and_level_from_frame"}
// Dependencies: {}
fn timestamp_and_level_from_frame (frame : & Frame < '_ >) -> (String , Option < Level >) { let timestamp = frame . display_timestamp () . map (| ts | ts . to_string ()) . unwrap_or_default () ; let level = frame . level () . map (| level | match level { crate :: Level :: Trace => Level :: Trace , crate :: Level :: Debug => Level :: Debug , crate :: Level :: Info => Level :: Info , crate :: Level :: Warn => Level :: Warn , crate :: Level :: Error => Level :: Error , }) ; (timestamp , level) }
};
}
