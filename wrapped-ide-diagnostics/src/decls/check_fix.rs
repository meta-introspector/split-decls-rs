macro_rules! check_fix {
    () => {
        # [doc = " Takes a multi-file input fixture with annotated cursor positions,"] # [doc = " and checks that:"] # [doc = "  * a diagnostic is produced"] # [doc = "  * the first diagnostic fix trigger range touches the input cursor position"] # [doc = "  * that the contents of the file containing the cursor match `after` after the diagnostic fix is applied"] # [track_caller] pub (crate) fn check_fix (# [rust_analyzer :: rust_fixture] ra_fixture_before : & str , # [rust_analyzer :: rust_fixture] ra_fixture_after : & str ,) { check_nth_fix (0 , ra_fixture_before , ra_fixture_after) ; }
    };
}

check_fix!()