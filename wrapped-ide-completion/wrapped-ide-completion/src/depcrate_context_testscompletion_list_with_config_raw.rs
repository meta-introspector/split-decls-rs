// Generated macro for completion_list_with_config_raw (function)
macro_rules! Depcrate_context_testscompletion_list_with_config_raw {
() => {
// Module: crate::context::tests
// Provides: {"completion_list_with_config_raw"}
// Dependencies: {}
fn completion_list_with_config_raw (config : CompletionConfig < '_ > , # [rust_analyzer :: rust_fixture] ra_fixture : & str , include_keywords : bool , trigger_character : Option < char > ,) -> Vec < CompletionItem > { let _tracing = setup_tracing () ; let items = get_all_items (config , ra_fixture , trigger_character) ; items . into_iter () . filter (| it | it . kind != CompletionItemKind :: BuiltinType || it . label . primary == "u32") . filter (| it | include_keywords || it . kind != CompletionItemKind :: Keyword) . filter (| it | include_keywords || it . kind != CompletionItemKind :: Snippet) . sorted_by_key (| it | { (it . kind , it . label . primary . clone () , it . label . detail_left . as_ref () . map (ToOwned :: to_owned) ,) }) . collect () }
};
}
