macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Many0 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] # [doc = " Parser implementation for the [many0] combinator"] pub struct Many0 < F > { parser : F , }
    };
}

Many0!();