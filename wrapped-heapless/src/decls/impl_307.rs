macro_rules! deps {
    () => {
        IntoIter!();
        LenType!();
    };
}

macro_rules! impl_307 {
    () => {
        deps!();
        impl < T , LenT : LenType , const N : usize > ExactSizeIterator for IntoIter < T , N , LenT > { fn len (& self) -> usize { (self . vec . len - self . next) . into_usize () } }
    };
}

impl_307!();