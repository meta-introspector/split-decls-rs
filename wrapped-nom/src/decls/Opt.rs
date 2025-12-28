macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Opt {
    () => {
        deps!();
        # [doc = " Parser implementation for [opt]"] pub struct Opt < F > { parser : F , }
    };
}

Opt!()