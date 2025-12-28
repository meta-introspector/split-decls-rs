macro_rules! deps {
    () => {
        Collector!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl Clone for Collector { # [doc = " Creates another reference to the same garbage collector."] fn clone (& self) -> Self { Self { global : self . global . clone () , } } }
    };
}

impl_71!()