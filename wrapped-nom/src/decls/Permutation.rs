macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Permutation {
    () => {
        deps!();
        # [doc = " Wrapping structure for the [permutation] combinator implementation"] pub struct Permutation < T , Error > { parser : T , e : PhantomData < Error > , }
    };
}

Permutation!()