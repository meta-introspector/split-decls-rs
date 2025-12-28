macro_rules! deps {
    () => {
        UrlParameter!();
        Key!();
        Section!();
        Tree!();
    };
}

macro_rules! impl_625 {
    () => {
        deps!();
        impl Section for UrlParameter { fn name (& self) -> & str { "<url>" } fn keys (& self) -> & [& dyn Key] { & [& Self :: HELPER , & Self :: USERNAME , & Self :: USE_HTTP_PATH] } fn parent (& self) -> Option < & dyn Section > { Some (& config :: Tree :: CREDENTIAL) } }
    };
}

impl_625!()