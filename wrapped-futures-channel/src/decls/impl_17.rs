macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < T > Node < T > { unsafe fn new (v : Option < T >) -> * mut Self { Box :: into_raw (Box :: new (Self { next : AtomicPtr :: new (ptr :: null_mut ()) , value : v })) } }
    };
}

impl_17!();