macro_rules! deps {
    () => {
        UnwindContextStorage!();
        UnwindContext!();
        ReaderOffset!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl < T , S > Default for UnwindContext < T , S > where T : ReaderOffset , S : UnwindContextStorage < T > , { fn default () -> Self { Self :: new_in () } }
    };
}

impl_216!();