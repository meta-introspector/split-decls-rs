macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Not {
    () => {
        deps!();
        # [doc = " Parser implementation for [not]"] pub struct Not < F > { parser : F , }
    };
}

Not!();