macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! LengthValue {
    () => {
        deps!();
        # [doc = " Parser implementation for the [length_value] combinator"] pub struct LengthValue < F , G , E > { length : F , parser : G , e : PhantomData < E > , }
    };
}

LengthValue!()