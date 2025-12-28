macro_rules! deps {
    () => {
        ReaderOffset!();
        UnwindContext!();
    };
}

macro_rules! impl_217 {
    () => {
        deps!();
        # [cfg (feature = "read")] impl < T : ReaderOffset > UnwindContext < T > { # [doc = " Construct a new call frame unwinding context."] pub fn new () -> Self { Self :: new_in () } }
    };
}

impl_217!()