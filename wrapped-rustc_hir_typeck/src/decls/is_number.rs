macro_rules! is_number {
    () => {
        fn is_number (text : & str) -> bool { text . chars () . all (| c : char | c . is_digit (10)) }
    };
}

is_number!();