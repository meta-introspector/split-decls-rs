macro_rules! deps {
    () => {
        Unstructured!();
        Arbitrary!();
        Result!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < 'a > Arbitrary < 'a > for Ipv4Addr { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { Ok (Ipv4Addr :: from (u32 :: arbitrary (u) ?)) } # [inline] fn size_hint (_depth : usize) -> (usize , Option < usize >) { (4 , Some (4)) } }
    };
}

impl_121!();