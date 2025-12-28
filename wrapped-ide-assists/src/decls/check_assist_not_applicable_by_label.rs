macro_rules! deps {
    () => {
        ExpectedResult!();
    };
}

macro_rules! check_assist_not_applicable_by_label {
    () => {
        deps!();
        # [track_caller] pub (crate) fn check_assist_not_applicable_by_label (assist : Handler , # [rust_analyzer :: rust_fixture] ra_fixture : & str , label : & str ,) { check (assist , ra_fixture , ExpectedResult :: NotApplicable , Some (label)) ; }
    };
}

check_assist_not_applicable_by_label!()