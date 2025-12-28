macro_rules! deps {
    () => {
        Unstructured!();
        Arbitrary!();
        Result!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < 'a > Arbitrary < 'a > for () { fn arbitrary (_ : & mut Unstructured < 'a >) -> Result < Self > { Ok (()) } # [inline] fn size_hint (_depth : usize) -> (usize , Option < usize >) { (0 , Some (0)) } }
    };
}

impl_109!()