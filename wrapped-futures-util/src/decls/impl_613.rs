macro_rules! deps {
    () => {
        TryNext!();
    };
}

macro_rules! impl_613 {
    () => {
        deps!();
        impl < 'a , St : ? Sized + TryStream + Unpin > TryNext < 'a , St > { pub (super) fn new (stream : & 'a mut St) -> Self { Self { stream } } }
    };
}

impl_613!()