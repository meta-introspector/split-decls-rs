macro_rules! deps {
    () => {
        Arbitrary!();
        Unstructured!();
        Result!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < 'a > Arbitrary < 'a > for IpAddr { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { if u . arbitrary () ? { Ok (IpAddr :: V4 (u . arbitrary () ?)) } else { Ok (IpAddr :: V6 (u . arbitrary () ?)) } } fn size_hint (depth : usize) -> (usize , Option < usize >) { size_hint :: and (bool :: size_hint (depth) , size_hint :: or (Ipv4Addr :: size_hint (depth) , Ipv6Addr :: size_hint (depth)) ,) } }
    };
}

impl_123!()