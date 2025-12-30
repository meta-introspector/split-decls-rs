// Generated macro for render_completion_list (function)
macro_rules! Depcrate_context_testsrender_completion_list {
() => {
// Module: crate::context::tests
// Provides: {"render_completion_list"}
// Dependencies: {}
fn render_completion_list (completions : Vec < CompletionItem >) -> String { fn monospace_width (s : & str) -> usize { s . chars () . count () } let label_width = completions . iter () . map (| it | { monospace_width (& it . label . primary) + monospace_width (it . label . detail_left . as_deref () . unwrap_or_default ()) + monospace_width (it . label . detail_right . as_deref () . unwrap_or_default ()) + it . label . detail_left . is_some () as usize + it . label . detail_right . is_some () as usize }) . max () . unwrap_or_default () ; completions . into_iter () . map (| it | { let tag = it . kind . tag () ; let mut buf = format ! ("{tag} {}" , it . label . primary) ; if let Some (label_detail) = & it . label . detail_left { format_to ! (buf , " {label_detail}") ; } if let Some (detail_right) = it . label . detail_right { let pad_with = label_width . saturating_sub (monospace_width (& it . label . primary) + monospace_width (it . label . detail_left . as_deref () . unwrap_or_default ()) + monospace_width (& detail_right) + it . label . detail_left . is_some () as usize ,) ; format_to ! (buf , "{:pad_with$}{detail_right}" , "" ,) ; } if it . deprecated { format_to ! (buf , " DEPRECATED") ; } format_to ! (buf , "\n") ; buf }) . collect () }
};
}
