macro_rules! deps {
    () => {
        AsyncSeekExt!();
    };
}

macro_rules! impl_1252 {
    () => {
        deps!();
        impl < S : AsyncSeek + ? Sized > AsyncSeekExt for S { }
    };
}

impl_1252!()