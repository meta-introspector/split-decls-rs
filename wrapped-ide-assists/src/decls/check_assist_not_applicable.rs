macro_rules! deps {
    () => {
        ExpectedResult!();
    };
}

macro_rules! check_assist_not_applicable {
    () => {
        deps!();
        # [track_caller] pub (crate) fn check_assist_not_applicable (assist : Handler , # [rust_analyzer :: rust_fixture] ra_fixture : & str ,) { check (assist , ra_fixture , ExpectedResult :: NotApplicable , None) ; }
    };
}

check_assist_not_applicable!()