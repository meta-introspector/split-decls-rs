macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Many0Count {
    () => {
        deps!();
        # [doc = " Parser implementation for the [many0_count] combinator"] pub struct Many0Count < F > { parser : F , }
    };
}

Many0Count!();