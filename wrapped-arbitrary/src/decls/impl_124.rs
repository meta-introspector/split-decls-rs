macro_rules! deps {
    () => {
        Arbitrary!();
        Unstructured!();
        Result!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < 'a > Arbitrary < 'a > for SocketAddrV4 { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { Ok (SocketAddrV4 :: new (u . arbitrary () ? , u . arbitrary () ?)) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { size_hint :: and (Ipv4Addr :: size_hint (depth) , u16 :: size_hint (depth)) } }
    };
}

impl_124!()