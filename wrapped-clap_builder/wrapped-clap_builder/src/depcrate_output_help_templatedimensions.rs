// Generated macro for dimensions (function)
macro_rules! Depcrate_output_help_templatedimensions {
() => {
// Module: crate::output::help_template
// Provides: {"dimensions"}
// Dependencies: {}
pub (crate) fn dimensions () -> (Option < usize > , Option < usize >) { # [cfg (not (feature = "wrap_help"))] return (None , None) ; # [cfg (feature = "wrap_help")] terminal_size :: terminal_size () . map (| (w , h) | (Some (w . 0 . into ()) , Some (h . 0 . into ()))) . unwrap_or_else (| | (parse_env ("COLUMNS") , parse_env ("LINES"))) }
};
}
