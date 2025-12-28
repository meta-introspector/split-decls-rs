macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! AnyChar {
    () => {
        deps!();
        # [doc = " Parser implementation for char"] pub struct AnyChar < E > { e : PhantomData < E > , }
    };
}

AnyChar!()