macro_rules! deps {
    () => {
        IntoChunks!();
    };
}

macro_rules! impl_275 {
    () => {
        deps!();
        impl < I > Debug for IntoChunks < I > where I : Iterator + Debug , I :: Item : Debug , { debug_fmt_fields ! (IntoChunks , inner , index) ; }
    };
}

impl_275!();