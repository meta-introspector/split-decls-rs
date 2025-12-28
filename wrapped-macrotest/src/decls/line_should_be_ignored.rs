macro_rules! line_should_be_ignored {
    () => {
        fn line_should_be_ignored (line : & str) -> bool { for check in IGNORED_LINES . iter () { if line . starts_with (check) { return true ; } } false }
    };
}

line_should_be_ignored!();