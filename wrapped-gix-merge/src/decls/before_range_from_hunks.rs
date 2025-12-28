macro_rules! deps {
    () => {
        Hunk!();
    };
}

macro_rules! before_range_from_hunks {
    () => {
        deps!();
        fn before_range_from_hunks (hunks : & [Hunk]) -> Range < u32 > { hunks . first () . zip (hunks . last ()) . map (| (f , l) | f . before . start .. l . before . end) . expect ("at least one entry") }
    };
}

before_range_from_hunks!()