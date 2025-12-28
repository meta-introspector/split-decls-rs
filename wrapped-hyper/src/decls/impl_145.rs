macro_rules! deps {
    () => {
        Response!();
        OnInformationalCallback!();
        OnInformationalClosure!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < F > OnInformationalCallback for OnInformationalClosure < F > where F : Fn (Response < '_ >) + Send + Sync + 'static , { fn on_informational (& self , res : http :: Response < () >) { let res = Response (& res) ; (self . 0) (res) ; } }
    };
}

impl_145!()