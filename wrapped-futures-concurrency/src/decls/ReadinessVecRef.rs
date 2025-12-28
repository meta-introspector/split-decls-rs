macro_rules! deps {
    () => {
        ReadinessVec!();
    };
}

macro_rules! ReadinessVecRef {
    () => {
        deps!();
        pub (crate) struct ReadinessVecRef < 'a > { inner : & 'a mut ReadinessVec , }
    };
}

ReadinessVecRef!()