macro_rules! completion_list_with_trigger_character {
    () => {
        pub (crate) fn completion_list_with_trigger_character (# [rust_analyzer :: rust_fixture] ra_fixture : & str , trigger_character : Option < char > ,) -> String { completion_list_with_config (TEST_CONFIG , ra_fixture , true , trigger_character) }
    };
}

completion_list_with_trigger_character!()