macro_rules! check_with_trigger_character {
    () => {
        pub (crate) fn check_with_trigger_character (# [rust_analyzer :: rust_fixture] ra_fixture : & str , trigger_character : Option < char > , expect : Expect ,) { let actual = completion_list_with_trigger_character (ra_fixture , trigger_character) ; expect . assert_eq (& actual) }
    };
}

check_with_trigger_character!();