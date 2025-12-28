macro_rules! deps {
    () => {
        Execs!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Drop for Execs { fn drop (& mut self) { if ! self . ran && ! std :: thread :: panicking () { panic ! ("forgot to run this command") ; } } }
    };
}

impl_29!()