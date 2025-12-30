// Generated macro for escape (function)
macro_rules! Depcrate_utils_step_graphescape {
() => {
// Module: crate::utils::step_graph
// Provides: {"escape"}
// Dependencies: {}
# [doc = " Normalizes the string so that it can be rendered into a DOT file."] fn escape (input : & str) -> String { input . replace ("\"" , "\\\"") }
};
}
