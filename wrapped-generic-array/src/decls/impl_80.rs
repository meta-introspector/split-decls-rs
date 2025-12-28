macro_rules! deps {
    () => {
        ArrayLength!();
        GAVisitor!();
        GenericArray!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < 'de , T , N : ArrayLength > Deserialize < 'de > for GenericArray < T , N > where T : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < GenericArray < T , N > , D :: Error > where D : Deserializer < 'de > , { let visitor = GAVisitor { _t : PhantomData , _n : PhantomData , } ; deserializer . deserialize_tuple (N :: USIZE , visitor) } }
    };
}

impl_80!();