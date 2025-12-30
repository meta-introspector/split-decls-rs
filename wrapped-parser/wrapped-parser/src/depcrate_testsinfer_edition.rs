// Generated macro for infer_edition (function)
macro_rules! Depcrate_testsinfer_edition {
() => {
// Module: crate::tests
// Provides: {"infer_edition"}
// Dependencies: {}
fn infer_edition (file_path : & Path) -> Edition { let file_content = std :: fs :: read_to_string (file_path) . unwrap () ; if let Some (edition) = file_content . strip_prefix ("//@ edition: ") { edition [.. 4] . parse () . expect ("invalid edition directive") } else { Edition :: CURRENT } }
};
}
