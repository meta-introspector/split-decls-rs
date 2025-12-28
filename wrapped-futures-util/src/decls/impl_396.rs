macro_rules! deps {
    () => {
        SelectNextSome!();
    };
}

macro_rules! impl_396 {
    () => {
        deps!();
        impl < 'a , St : ? Sized > SelectNextSome < 'a , St > { pub (super) fn new (stream : & 'a mut St) -> Self { Self { stream } } }
    };
}

impl_396!()