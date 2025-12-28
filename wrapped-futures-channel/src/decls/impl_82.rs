macro_rules! deps {
    () => {
        Recv!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < 'a , St : ? Sized + Stream + Unpin > Recv < 'a , St > { fn new (stream : & 'a mut St) -> Self { Self { stream } } }
    };
}

impl_82!();