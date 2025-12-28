macro_rules! deps {
    () => {
        PunycodeCaller!();
        PunycodeCodeUnit!();
        Decode!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < T : PunycodeCodeUnit + Copy , C : PunycodeCaller > ExactSizeIterator for Decode < '_ , T , C > { fn len (& self) -> usize { self . len - self . position } }
    };
}

impl_33!()