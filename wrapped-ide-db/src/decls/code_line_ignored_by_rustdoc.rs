macro_rules! code_line_ignored_by_rustdoc {
    () => {
        fn code_line_ignored_by_rustdoc (line : & str) -> bool { let trimmed = line . trim () ; trimmed == "#" || trimmed . starts_with ("# ") || trimmed . starts_with ("#\t") }
    };
}

code_line_ignored_by_rustdoc!();