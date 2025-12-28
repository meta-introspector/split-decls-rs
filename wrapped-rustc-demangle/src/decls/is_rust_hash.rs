macro_rules! is_rust_hash {
    () => {
        fn is_rust_hash (s : & str) -> bool { s . starts_with ('h') && s [1 ..] . chars () . all (| c | c . is_digit (16)) }
    };
}

is_rust_hash!()