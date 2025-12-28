macro_rules! deps {
    () => {
        AbortOnPanic!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl Drop for AbortOnPanic { fn drop (& mut self) { if std :: thread :: panicking () { std :: process :: abort () } } }
    };
}

impl_140!();