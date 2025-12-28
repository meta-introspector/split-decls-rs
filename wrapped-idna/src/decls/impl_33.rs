macro_rules! deps {
    () => {
        PunycodeCodeUnit!();
        PunycodeCaller!();
        Decode!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < T : PunycodeCodeUnit + Copy , C : PunycodeCaller > ExactSizeIterator for Decode < '_ , T , C > { fn len (& self) -> usize { self . len - self . position } }
    };
}

impl_33!();