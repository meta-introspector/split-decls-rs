macro_rules! deps {
    () => {
        ExpectedResult!();
    };
}

macro_rules! check_assist_target {
    () => {
        deps!();
        # [track_caller] pub (crate) fn check_assist_target (assist : Handler , # [rust_analyzer :: rust_fixture] ra_fixture : & str , target : & str ,) { check (assist , ra_fixture , ExpectedResult :: Target (target) , None) ; }
    };
}

check_assist_target!()