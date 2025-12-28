macro_rules! deps {
    () => {
        Arbitrary!();
        Unstructured!();
        Result!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < 'a , A > Arbitrary < 'a > for PhantomData < A > where A : ? Sized , { fn arbitrary (_ : & mut Unstructured < 'a >) -> Result < Self > { Ok (PhantomData) } # [inline] fn size_hint (_depth : usize) -> (usize , Option < usize >) { (0 , Some (0)) } }
    };
}

impl_56!()