macro_rules! deps {
    () => {
        BenchmarkId!();
        Result!();
    };
}

macro_rules! impl_269 {
    () => {
        deps!();
        impl fmt :: Display for BenchmarkId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (self . as_title ()) } }
    };
}

impl_269!()