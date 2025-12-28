macro_rules! shuffle {
    () => {
        # [doc = " Shuffles a slice randomly."] # [inline] pub fn shuffle < T > (slice : & mut [T]) { with_rng (| r | r . shuffle (slice)) }
    };
}

shuffle!()