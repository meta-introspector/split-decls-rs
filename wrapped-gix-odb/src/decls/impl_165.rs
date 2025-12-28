macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl Sink { # [doc = " Enable or disable compression. Compression is disabled by default"] pub fn compress (mut self , enable : bool) -> Self { if enable { self . compressor = Some (RefCell :: new (deflate :: Write :: new (io :: sink ()))) ; } else { self . compressor = None ; } self } }
    };
}

impl_165!();