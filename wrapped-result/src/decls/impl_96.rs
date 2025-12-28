macro_rules! deps {
    () => {
        BOOL!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl core :: ops :: Not for BOOL { type Output = Self ; fn not (self) -> Self :: Output { if self . as_bool () { Self (0) } else { Self (1) } } }
    };
}

impl_96!()