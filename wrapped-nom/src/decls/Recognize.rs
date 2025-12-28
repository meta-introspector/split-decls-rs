macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Recognize {
    () => {
        deps!();
        # [doc = " Parser implementation for [recognize]"] pub struct Recognize < F > { parser : F , }
    };
}

Recognize!();