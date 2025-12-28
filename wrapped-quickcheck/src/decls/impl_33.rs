macro_rules! deps {
    () => {
        Arbitrary!();
        Gen!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl Arbitrary for SocketAddrV4 { fn arbitrary (g : & mut Gen) -> SocketAddrV4 { SocketAddrV4 :: new (Arbitrary :: arbitrary (g) , g . random ()) } }
    };
}

impl_33!();