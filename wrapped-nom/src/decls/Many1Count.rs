macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Many1Count {
    () => {
        deps!();
        # [doc = " Parser implementation for the [many1_count] combinator"] pub struct Many1Count < F > { parser : F , }
    };
}

Many1Count!()