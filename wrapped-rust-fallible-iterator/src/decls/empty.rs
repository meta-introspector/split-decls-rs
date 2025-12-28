macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! empty {
    () => {
        deps!();
        # [doc = " Creates an iterator that yields nothing."] pub fn empty < T , E > () -> Empty < T , E > { Empty (PhantomData , PhantomData) }
    };
}

empty!();