macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Cond {
    () => {
        deps!();
        # [doc = " Parser implementation for [cond]"] pub struct Cond < F > { parser : Option < F > , }
    };
}

Cond!()