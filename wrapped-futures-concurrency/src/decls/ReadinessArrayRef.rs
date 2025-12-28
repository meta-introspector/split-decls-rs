macro_rules! deps {
    () => {
        ReadinessArray!();
    };
}

macro_rules! ReadinessArrayRef {
    () => {
        deps!();
        pub (crate) struct ReadinessArrayRef < 'a , const N : usize > { inner : & 'a mut ReadinessArray < N > , }
    };
}

ReadinessArrayRef!()