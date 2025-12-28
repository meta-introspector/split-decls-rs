macro_rules! unhex_a {
    () => {
        # [inline] fn unhex_a (x : usize) -> u8 { UNHEX4 [x] }
    };
}

unhex_a!();