macro_rules! deps {
    () => {
        Gen!();
        Arbitrary!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Arbitrary for IpAddr { fn arbitrary (g : & mut Gen) -> IpAddr { let ipv4 : bool = g . random () ; if ipv4 { IpAddr :: V4 (Arbitrary :: arbitrary (g)) } else { IpAddr :: V6 (Arbitrary :: arbitrary (g)) } } }
    };
}

impl_29!()