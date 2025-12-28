macro_rules! deps {
    () => {
        ExpectedResult!();
        AssistConfig!();
    };
}

macro_rules! check_assist_with_config {
    () => {
        deps!();
        # [track_caller] pub (crate) fn check_assist_with_config (assist : Handler , config : AssistConfig , # [rust_analyzer :: rust_fixture] ra_fixture_before : & str , # [rust_analyzer :: rust_fixture] ra_fixture_after : & str ,) { let ra_fixture_after = trim_indent (ra_fixture_after) ; check_with_config (config , assist , ra_fixture_before , ExpectedResult :: After (& ra_fixture_after) , None ,) ; }
    };
}

check_assist_with_config!();