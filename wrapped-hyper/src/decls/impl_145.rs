macro_rules! deps {
    () => {
        OnInformationalCallback!();
        OnInformationalClosure!();
        Response!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < F > OnInformationalCallback for OnInformationalClosure < F > where F : Fn (Response < '_ >) + Send + Sync + 'static , { fn on_informational (& self , res : http :: Response < () >) { let res = Response (& res) ; (self . 0) (res) ; } }
    };
}

impl_145!();