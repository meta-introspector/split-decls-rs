macro_rules! deps {
    () => {
        Generator!();
    };
}

macro_rules! Gn {
    () => {
        deps!();
        # [doc = " Generator helper"] pub struct Gn < A = () > { dummy : PhantomData < A > , }
    };
}

Gn!();