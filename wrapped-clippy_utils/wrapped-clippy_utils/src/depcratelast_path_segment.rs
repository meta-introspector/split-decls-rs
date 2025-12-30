// Generated macro for last_path_segment (function)
macro_rules! Depcratelast_path_segment {
() => {
// Module: crate
// Provides: {"last_path_segment"}
// Dependencies: {}
pub fn last_path_segment < 'tcx > (path : & QPath < 'tcx >) -> & 'tcx PathSegment < 'tcx > { match * path { QPath :: Resolved (_ , path) => path . segments . last () . expect ("A path must have at least one segment") , QPath :: TypeRelative (_ , seg) => seg , } }
};
}
