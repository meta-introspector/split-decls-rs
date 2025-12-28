macro_rules! deps {
    () => {
        Snapshot!();
    };
}

macro_rules! impl_555 {
    () => {
        deps!();
        impl Debug for Snapshot < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . write_str (& self . repo . config . resolved . to_string ()) } }
    };
}

impl_555!();