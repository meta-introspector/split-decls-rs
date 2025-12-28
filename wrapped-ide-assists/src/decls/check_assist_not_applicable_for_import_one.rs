macro_rules! deps {
    () => {
        ExpectedResult!();
    };
}

macro_rules! check_assist_not_applicable_for_import_one {
    () => {
        deps!();
        # [track_caller] pub (crate) fn check_assist_not_applicable_for_import_one (assist : Handler , # [rust_analyzer :: rust_fixture] ra_fixture : & str ,) { check_with_config (TEST_CONFIG_IMPORT_ONE , assist , ra_fixture , ExpectedResult :: NotApplicable , None ,) ; }
    };
}

check_assist_not_applicable_for_import_one!();