macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Fill {
    () => {
        deps!();
        # [doc = " Parser implementation for the [fill] combinator"] pub struct Fill < 'a , F , O > { parser : F , buf : & 'a mut [O] , }
    };
}

Fill!()