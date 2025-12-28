macro_rules! deps {
    () => {
        LatchRef!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < L > LatchRef < '_ , L > { pub (super) fn new (inner : & L) -> LatchRef < '_ , L > { LatchRef { inner , marker : PhantomData , } } }
    };
}

impl_94!()