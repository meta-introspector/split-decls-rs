macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Digit1 {
    () => {
        deps!();
        # [doc = " Parser implementation for [digit1]"] pub struct Digit1 < E > { e : PhantomData < E > , }
    };
}

Digit1!()