macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! SeparatedList1 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] # [doc = " Parser implementation for the [separated_list1] combinator"] pub struct SeparatedList1 < F , G > { parser : F , separator : G , }
    };
}

SeparatedList1!()