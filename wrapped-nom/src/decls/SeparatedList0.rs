macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! SeparatedList0 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] # [doc = " Parser implementation for the [separated_list0] combinator"] pub struct SeparatedList0 < F , G > { parser : F , separator : G , }
    };
}

SeparatedList0!();