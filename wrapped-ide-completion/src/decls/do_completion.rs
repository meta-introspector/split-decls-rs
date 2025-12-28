macro_rules! deps {
    () => {
        CompletionItemKind!();
        CompletionItem!();
    };
}

macro_rules! do_completion {
    () => {
        deps!();
        pub (crate) fn do_completion (code : & str , kind : CompletionItemKind) -> Vec < CompletionItem > { do_completion_with_config (TEST_CONFIG , code , kind) }
    };
}

do_completion!()