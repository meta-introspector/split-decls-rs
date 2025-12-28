macro_rules! deps {
    () => {
        State!();
        WaitUntil!();
    };
}

macro_rules! impl_384 {
    () => {
        deps!();
        impl < F , D > WaitUntil < F , D > { pub (super) fn new (future : F , deadline : D) -> Self { Self { future , deadline , state : State :: Started , } } }
    };
}

impl_384!();