macro_rules! deps {
    () => {
        Result!();
        Unstructured!();
        Arbitrary!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < 'a > Arbitrary < 'a > for Ipv6Addr { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { Ok (Ipv6Addr :: from (u128 :: arbitrary (u) ?)) } # [inline] fn size_hint (_depth : usize) -> (usize , Option < usize >) { (16 , Some (16)) } }
    };
}

impl_122!()