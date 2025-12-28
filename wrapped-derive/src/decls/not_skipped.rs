macro_rules! not_skipped {
    () => {
        pub fn not_skipped (variant : & & Variant) -> bool { ! should_skip (variant) }
    };
}

not_skipped!()