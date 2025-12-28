macro_rules! deps {
    () => {
        ExpectedResult!();
    };
}

macro_rules! check_assist_by_label {
    () => {
        deps!();
        # [track_caller] pub (crate) fn check_assist_by_label (assist : Handler , # [rust_analyzer :: rust_fixture] ra_fixture_before : & str , # [rust_analyzer :: rust_fixture] ra_fixture_after : & str , label : & str ,) { let ra_fixture_after = trim_indent (ra_fixture_after) ; check (assist , ra_fixture_before , ExpectedResult :: After (& ra_fixture_after) , Some (label)) ; }
    };
}

check_assist_by_label!()