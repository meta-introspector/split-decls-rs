macro_rules! deps {
    () => {
        Once!();
    };
}

macro_rules! once {
    () => {
        deps!();
        # [doc = " Creates an iterator that yields an element exactly once."] pub fn once < T , E > (value : T) -> Once < T , E > { Once (Some (value) , PhantomData) }
    };
}

once!();