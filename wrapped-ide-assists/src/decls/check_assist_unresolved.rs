macro_rules! deps {
    () => {
        ExpectedResult!();
    };
}

macro_rules! check_assist_unresolved {
    () => {
        deps!();
        # [doc = " Check assist in unresolved state. Useful to check assists for lazy computation."] # [track_caller] pub (crate) fn check_assist_unresolved (assist : Handler , # [rust_analyzer :: rust_fixture] ra_fixture : & str ,) { check (assist , ra_fixture , ExpectedResult :: Unresolved , None) ; }
    };
}

check_assist_unresolved!()