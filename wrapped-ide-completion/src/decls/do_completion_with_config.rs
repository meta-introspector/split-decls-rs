macro_rules! deps {
    () => {
        CompletionConfig!();
        CompletionItemKind!();
        CompletionItem!();
    };
}

macro_rules! do_completion_with_config {
    () => {
        deps!();
        pub (crate) fn do_completion_with_config (config : CompletionConfig < '_ > , code : & str , kind : CompletionItemKind ,) -> Vec < CompletionItem > { get_all_items (config , code , None) . into_iter () . filter (| c | c . kind == kind) . sorted_by (| l , r | l . label . cmp (& r . label)) . collect () }
    };
}

do_completion_with_config!()