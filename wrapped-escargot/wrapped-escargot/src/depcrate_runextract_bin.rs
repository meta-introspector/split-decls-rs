// Generated macro for extract_bin (function)
macro_rules! Depcrate_runextract_bin {
() => {
// Module: crate::run
// Provides: {"extract_bin"}
// Dependencies: {}
fn extract_bin < 'a > (msg : & 'a format :: Message < '_ > , desired_kind : & str) -> Option < & 'a path :: Path > { match msg { format :: Message :: CompilerArtifact (art) => { if ! art . profile . test && art . target . crate_types == ["bin"] && art . target . kind == [desired_kind] { Some (art . filenames . first () . expect ("files must exist")) } else { None } } _ => None , } }
};
}
