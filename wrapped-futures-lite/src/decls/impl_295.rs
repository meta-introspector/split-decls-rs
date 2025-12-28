macro_rules! deps {
    () => {
        AsyncSeekExt!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        impl < S : AsyncSeek + ? Sized > AsyncSeekExt for S { }
    };
}

impl_295!();