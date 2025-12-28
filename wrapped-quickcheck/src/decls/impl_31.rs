macro_rules! deps {
    () => {
        Gen!();
        Arbitrary!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl Arbitrary for Ipv6Addr { fn arbitrary (g : & mut Gen) -> Ipv6Addr { Ipv6Addr :: new (g . random () , g . random () , g . random () , g . random () , g . random () , g . random () , g . random () , g . random () ,) } }
    };
}

impl_31!();