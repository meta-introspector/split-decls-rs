macro_rules! deps {
    () => {
        RawIdx!();
    };
}

macro_rules! Idx {
    () => {
        deps!();
        # [doc = " The index of a value allocated in an arena that holds `T`s."] pub struct Idx < T > { raw : RawIdx , _ty : PhantomData < fn () -> T > , }
    };
}

Idx!()