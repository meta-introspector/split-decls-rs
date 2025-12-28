macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Cut {
    () => {
        deps!();
        # [doc = " Parser implementation for [cut]"] pub struct Cut < F > { parser : F , }
    };
}

Cut!();