macro_rules! deps {
    () => {
        Arbitrary!();
        Gen!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl Arbitrary for SocketAddr { fn arbitrary (g : & mut Gen) -> SocketAddr { SocketAddr :: new (Arbitrary :: arbitrary (g) , g . random ()) } }
    };
}

impl_32!();