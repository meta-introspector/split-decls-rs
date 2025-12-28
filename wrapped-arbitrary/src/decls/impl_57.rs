macro_rules! deps {
    () => {
        Arbitrary!();
        Unstructured!();
        Result!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < 'a > Arbitrary < 'a > for PhantomPinned { fn arbitrary (_ : & mut Unstructured < 'a >) -> Result < Self > { Ok (PhantomPinned) } # [inline] fn size_hint (_depth : usize) -> (usize , Option < usize >) { (0 , Some (0)) } }
    };
}

impl_57!()