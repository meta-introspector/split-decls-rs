macro_rules! is_inner {
    () => {
        fn is_inner (text : & str) -> bool { text . starts_with ("//!") || text . starts_with ("/*!") }
    };
}

is_inner!();