macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Take {
    () => {
        deps!();
        # [doc = " Parser implementation for [take]"] pub struct Take < E > { length : usize , e : PhantomData < E > , }
    };
}

Take!();