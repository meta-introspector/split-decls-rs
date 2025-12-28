macro_rules! is_ascii_whitespace {
    () => {
        # [inline] fn is_ascii_whitespace (c : char) -> bool { c . is_ascii_whitespace () }
    };
}

is_ascii_whitespace!();