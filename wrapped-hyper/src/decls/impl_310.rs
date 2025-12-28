macro_rules! deps {
    () => {
        Response!();
        OnInformational!();
        OnInformationalCallback!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        # [cfg (feature = "client")] impl crate :: ext :: OnInformationalCallback for OnInformational { fn on_informational (& self , res : http :: Response < () >) { let res = res . map (| () | IncomingBody :: empty ()) ; let mut res = hyper_response :: wrap (res) ; (self . func) (self . data . 0 , & mut res) ; } }
    };
}

impl_310!();