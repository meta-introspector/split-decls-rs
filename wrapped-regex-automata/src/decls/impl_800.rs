macro_rules! deps {
    () => {
        SerializeError!();
    };
}

macro_rules! impl_800 {
    () => {
        deps!();
        impl SerializeError { pub (crate) fn buffer_too_small (what : & 'static str) -> SerializeError { SerializeError { what } } }
    };
}

impl_800!()