macro_rules! deps {
    () => {
        Arbitrary!();
        Gen!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl Arbitrary for Ipv4Addr { fn arbitrary (g : & mut Gen) -> Ipv4Addr { Ipv4Addr :: new (g . random () , g . random () , g . random () , g . random ()) } }
    };
}

impl_30!()