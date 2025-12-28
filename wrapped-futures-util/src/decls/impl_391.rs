macro_rules! deps {
    () => {
        Next!();
    };
}

macro_rules! impl_391 {
    () => {
        deps!();
        impl < 'a , St : ? Sized + Stream + Unpin > Next < 'a , St > { pub (super) fn new (stream : & 'a mut St) -> Self { Self { stream } } }
    };
}

impl_391!();