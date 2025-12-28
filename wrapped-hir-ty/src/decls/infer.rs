macro_rules! infer {
    () => {
        fn infer (# [rust_analyzer :: rust_fixture] ra_fixture : & str) -> String { infer_with_mismatches (ra_fixture , false) }
    };
}

infer!()