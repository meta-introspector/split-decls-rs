macro_rules! fill {
    () => {
        # [doc = " Fill a byte slice with random data."] # [inline] pub fn fill (slice : & mut [u8]) { with_rng (| r | r . fill (slice)) }
    };
}

fill!()