macro_rules! deps {
    () => {
        Repeat!();
    };
}

macro_rules! repeat {
    () => {
        deps!();
        # [doc = " Creates an iterator that endlessly repeats a single element."] pub fn repeat < T : Clone , E > (value : T) -> Repeat < T , E > { Repeat (value , PhantomData) }
    };
}

repeat!();