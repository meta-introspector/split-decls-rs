macro_rules! deps {
    () => {
        Arbitrary!();
        Unstructured!();
        Result!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < 'a > Arbitrary < 'a > for & 'a str { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { let size = u . arbitrary_len :: < u8 > () ? ; arbitrary_str (u , size) } fn arbitrary_take_rest (mut u : Unstructured < 'a >) -> Result < Self > { let size = u . len () ; arbitrary_str (& mut u , size) } # [inline] fn size_hint (_depth : usize) -> (usize , Option < usize >) { (0 , None) } }
    };
}

impl_97!();