macro_rules! deps {
    () => {
        Response!();
    };
}

macro_rules! OnInformationalCallback {
    () => {
        deps!();
        pub (crate) trait OnInformationalCallback { fn on_informational (& self , res : http :: Response < () >) ; }
    };
}

OnInformationalCallback!();