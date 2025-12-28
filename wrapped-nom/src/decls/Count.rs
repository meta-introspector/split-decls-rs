macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Count {
    () => {
        deps!();
        # [cfg (feature = "alloc")] # [doc = " Parser implementation for the [count] combinator"] pub struct Count < F > { parser : F , count : usize , }
    };
}

Count!();