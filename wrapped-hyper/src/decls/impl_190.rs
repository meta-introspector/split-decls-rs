macro_rules! deps {
    () => {
        Service!();
        Response!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl < T , B1 , B2 > sealed :: Sealed < B1 > for T where T : Service < Request < B1 > , Response = Response < B2 > > , B2 : Body , { }
    };
}

impl_190!();