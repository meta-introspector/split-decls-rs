macro_rules! deps {
    () => {
        WaitUntil!();
        State!();
    };
}

macro_rules! impl_472 {
    () => {
        deps!();
        impl < S , D > WaitUntil < S , D > { pub (crate) fn new (stream : S , deadline : D) -> Self { WaitUntil { stream , deadline , state : State :: Timer , } } }
    };
}

impl_472!();