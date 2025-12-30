// Generated macro for do_completion_with_config (function)
macro_rules! Depcrate_context_testsdo_completion_with_config {
() => {
// Module: crate::context::tests
// Provides: {"do_completion_with_config"}
// Dependencies: {}
pub (crate) fn do_completion_with_config (config : CompletionConfig < '_ > , code : & str , kind : CompletionItemKind ,) -> Vec < CompletionItem > { get_all_items (config , code , None) . into_iter () . filter (| c | c . kind == kind) . sorted_by (| l , r | l . label . cmp (& r . label)) . collect () }
};
}
