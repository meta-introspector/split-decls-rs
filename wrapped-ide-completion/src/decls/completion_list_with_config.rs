macro_rules! deps {
    () => {
        CompletionConfig!();
    };
}

macro_rules! completion_list_with_config {
    () => {
        deps!();
        fn completion_list_with_config (config : CompletionConfig < '_ > , # [rust_analyzer :: rust_fixture] ra_fixture : & str , include_keywords : bool , trigger_character : Option < char > ,) -> String { render_completion_list (completion_list_with_config_raw (config , ra_fixture , include_keywords , trigger_character ,)) }
    };
}

completion_list_with_config!();