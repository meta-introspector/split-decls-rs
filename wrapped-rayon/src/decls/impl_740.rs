macro_rules! deps {
    () => {
        PanicFuse!();
    };
}

macro_rules! impl_740 {
    () => {
        deps!();
        impl < I > PanicFuse < I > { # [doc = " Creates a new `PanicFuse` iterator."] pub (super) fn new (base : I) -> PanicFuse < I > { PanicFuse { base } } }
    };
}

impl_740!()