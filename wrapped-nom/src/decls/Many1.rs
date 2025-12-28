macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Many1 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] # [doc = " Parser implementation for the [many1] combinator"] pub struct Many1 < F > { parser : F , }
    };
}

Many1!()