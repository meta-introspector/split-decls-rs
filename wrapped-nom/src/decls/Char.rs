macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Char {
    () => {
        deps!();
        # [doc = " Parser implementation for [char()]"] pub struct Char < E > { c : char , e : PhantomData < E > , }
    };
}

Char!()