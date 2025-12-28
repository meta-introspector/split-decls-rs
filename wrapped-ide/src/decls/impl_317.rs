macro_rules! deps {
    () => {
        MonikerResult!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        impl MonikerResult { pub fn from_def (db : & RootDatabase , def : Definition , from_crate : Crate) -> Option < Self > { def_to_moniker (db , def , from_crate) } }
    };
}

impl_317!()