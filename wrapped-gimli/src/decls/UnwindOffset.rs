macro_rules! deps {
    () => {
        ReaderOffset!();
        UnwindSection!();
    };
}

macro_rules! UnwindOffset {
    () => {
        deps!();
        # [doc = " An offset into an `UnwindSection`."] pub trait UnwindOffset < T = usize > : Copy + Debug + Eq + From < T > where T : ReaderOffset , { # [doc = " Convert an `UnwindOffset<T>` into a `T`."] fn into (self) -> T ; }
    };
}

UnwindOffset!()