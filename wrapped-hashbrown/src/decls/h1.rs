macro_rules! h1 {
    () => {
        # [doc = " Primary hash function, used to select the initial bucket to probe from."] # [inline] # [allow (clippy :: cast_possible_truncation)] fn h1 (hash : u64) -> usize { hash as usize }
    };
}

h1!()