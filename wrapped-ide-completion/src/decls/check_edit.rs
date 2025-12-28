macro_rules! check_edit {
    () => {
        # [track_caller] pub (crate) fn check_edit (what : & str , # [rust_analyzer :: rust_fixture] ra_fixture_before : & str , # [rust_analyzer :: rust_fixture] ra_fixture_after : & str ,) { check_edit_with_config (TEST_CONFIG , what , ra_fixture_before , ra_fixture_after) }
    };
}

check_edit!();