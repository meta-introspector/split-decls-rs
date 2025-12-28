macro_rules! deps {
    () => {
        RingElementNTT!();
    };
}

macro_rules! impl_434 {
    () => {
        deps!();
        impl Zeroize for RingElementNTT { fn zeroize (& mut self) { self . coefficients . iter_mut () . zeroize () ; } }
    };
}

impl_434!()