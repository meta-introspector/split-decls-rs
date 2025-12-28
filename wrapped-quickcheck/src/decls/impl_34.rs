macro_rules! deps {
    () => {
        Arbitrary!();
        Gen!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl Arbitrary for SocketAddrV6 { fn arbitrary (g : & mut Gen) -> SocketAddrV6 { SocketAddrV6 :: new (Arbitrary :: arbitrary (g) , g . random () , g . random () , g . random () ,) } }
    };
}

impl_34!();