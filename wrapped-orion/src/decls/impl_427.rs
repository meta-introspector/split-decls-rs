macro_rules! deps {
    () => {
        RingElement!();
    };
}

macro_rules! impl_427 {
    () => {
        deps!();
        impl Zeroize for RingElement { fn zeroize (& mut self) { self . coefficients . iter_mut () . zeroize () ; } }
    };
}

impl_427!()