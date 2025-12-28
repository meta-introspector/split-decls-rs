macro_rules! deps {
    () => {
        Io!();
        Result!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl dyn Io + Send { fn __hyper_is < T : Io > (& self) -> bool { let t = TypeId :: of :: < T > () ; self . __hyper_type_id () == t } fn __hyper_downcast < T : Io > (self : Box < Self >) -> Result < Box < T > , Box < Self > > { if self . __hyper_is :: < T > () { unsafe { let raw : * mut dyn Io = Box :: into_raw (self) ; Ok (Box :: from_raw (raw as * mut T)) } } else { Err (self) } } }
    };
}

impl_227!()